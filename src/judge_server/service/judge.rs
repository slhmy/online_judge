use crate::{
    database::*,
    errors::{ServiceError, ServiceResult},
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct TestCaseResult {
    test_case: String,
    result: String,
    error: String,
    cpu_time: i32,
    real_time: i32,
    memory: i32,
    signal: i32,
    exit_code: i32,
    output_md5: Option<String>,
    output: Option<String>,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct OutJudgeResult {
    pub judge_type: String,
    pub result: String,
    pub err_msg: Option<String>,
    pub data: Option<Vec<TestCaseResult>>,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct ProblemSetting {
    pub default_max_cpu_time: i32,
    pub default_max_memory: i32,
    pub is_spj: bool,
    pub opaque_output: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSettingMessage {
    pub region: String,
    pub problem_id: i32,
}

impl Message for GetSettingMessage {
    type Result = Result<ProblemSetting, String>;
}

impl Handler<GetSettingMessage> for DbExecutor {
    type Result = Result<ProblemSetting, String>;
    
    fn handle(&mut self, msg: GetSettingMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::problems::dsl::*;
        use crate::schema::test_cases::dsl::*;

        let (default_max_cpu_time_val, default_max_memory_val, test_case_name, opaque_output_val) = problems
            .filter(region.eq(msg.region))
            .filter(id.eq(msg.problem_id))
            .select( (default_max_cpu_time, default_max_memory, test_case, opaque_output) )
            .first::<(i32, i32, Option<String>, bool)>(&self.0)
            .expect("Error loading problem setting.");

        info!("{:?}", test_case_name);
        if test_case_name.is_none() { return Err("Problem doesn't have test cases.".to_owned()) }
        let is_spj_val = test_cases.filter(name.eq(test_case_name.unwrap()))
            .select(is_spj)
            .first::<bool>(&self.0)
            .expect("Error loading test case info.");

        Ok(ProblemSetting{
            default_max_cpu_time: default_max_cpu_time_val,
            default_max_memory: default_max_memory_val,
            is_spj: is_spj_val,
            opaque_output: opaque_output_val,
        })
    }
}

async fn get_judge_result(
    region: String,
    problem_id: i32,
    language: String,
    src: String,
    is_spj: bool,
    max_cpu_time: i32,
    max_memory: i32,
    judge_type: String,
    output: bool,
) -> Result<OutJudgeResult, String> {
    use crate::judge_server::utils::{
        builder::get_judge_setting,
    };
    use crate::judge_server:: { model::*, utils::mapper::*, utils::chooser::* };
    use actix_web::client::Client;
    use std::str;    

    let judge_server = choose_judge_server();
    if judge_server.is_none() { return Err("No judge server is online".to_owned()); }
    let (url, token) = judge_server.unwrap();

    let judge_setting = {
        match get_judge_setting(
            region,
            problem_id,
            language,
            src,
            is_spj,
            max_cpu_time,
            max_memory,
            output,
        ) {
            Err(msg) => { return Err(msg); }
            Ok(judge_setting) => judge_setting
        }
    };

    let mut response = Client::new()
        .post(url + "/judge")
        .set_header("X-Judge-Server-Token", token)
        .set_header("Content-Type", "application/json")
        .timeout(Duration::new(30, 0))
        .send_json(&judge_setting)
        .await.expect("Error sending judge msg to judge server.");
    let result_vec = response.body().await.expect("Error getting response body.").to_vec();
    let result_str = str::from_utf8(&result_vec).unwrap();
    
    let err_checker: ErrChecker = serde_json::from_str(result_str).unwrap();
    if err_checker.err.is_none() {
        let judge_result: JudgeResult = serde_json::from_str(result_str).unwrap();
        let mut final_result = "Accepted".to_owned();
        let mut total_test_cases = 0;
        let mut passed_test_cases = 0;
        let mut test_case_results: Vec<TestCaseResult> = Vec::new();
        for judge_result_data in judge_result.data {
            total_test_cases += 1;
            test_case_results.push(TestCaseResult {
                test_case: judge_result_data.test_case,
                result: result_mapper(judge_result_data.result),
                error: err_mapper(judge_result_data.error),
                cpu_time: judge_result_data.cpu_time,
                real_time: judge_result_data.real_time,
                memory: judge_result_data.memory,
                signal: judge_result_data.signal,
                exit_code: judge_result_data.exit_code,
                output_md5: judge_result_data.output_md5,
                output: judge_result_data.output,
            });
            if result_mapper(judge_result_data.result) != "SUCCESS".to_owned() {
                final_result = "Unaccepted".to_owned()
            } else {
                passed_test_cases += 1;
            }
        }
        match judge_type.as_str() {
            "OI" => { final_result = (100.0 * (passed_test_cases as f32 / total_test_cases as f32)).to_string() },
            _ => {},
        }
        Ok(
            OutJudgeResult {
                judge_type: judge_type,
                result: final_result,
                err_msg: None,
                data: Some(test_case_results),
            }
        )
    } else {
        let err_result: ErrResult = serde_json::from_str(result_str).unwrap();
        Ok(
            OutJudgeResult {
                judge_type: judge_type,
                result: err_result.err.unwrap(),
                err_msg: Some(err_result.data),
                data: None,
            }
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JudgeRequestForm {
    pub problem_id: i32,
    pub region: String,
    pub src: String,
    pub language: String,
    pub judge_type: String,
}

pub async fn judge(
    data: web::Data<DBState>,
    problem_id: i32,
    region: String,
    src: String,
    language: String,
    judge_type: String,
    _id: Identity,
) -> ServiceResult<OutJudgeResult> {
    use crate::judge_server::utils::filter::*; 

    if !language_filter(&language) {
        return Err(ServiceError::BadRequest("Language doesn't support.".to_owned()))
    }

    if !judge_type_filter(&judge_type) {
        return Err(ServiceError::BadRequest("JudgeType doesn't support.".to_owned()))
    }
    // get setting
    let db_result = data.db.send(GetSettingMessage {
        region: region.clone(),
        problem_id: problem_id,
    }).await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(problem_setting) => {
                    let default_max_cpu_time = problem_setting.default_max_cpu_time;
                    let default_max_memory = problem_setting.default_max_memory;
                    let (max_cpu_time, max_memory) = setting_filter(&language, default_max_cpu_time, default_max_memory);
                    let judge_result = get_judge_result(
                        region,
                        problem_id,
                        language,
                        src,
                        problem_setting.is_spj,
                        max_cpu_time,
                        max_memory,
                        judge_type,
                        true,
                    ).await;
                    
                    match judge_result {
                        Err(msg) => Err(ServiceError::BadRequest(msg)),
                        Ok(inner_result) => Ok(inner_result),
                    }
                },
            }
        }
    }
}