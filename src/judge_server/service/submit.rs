use crate::{
    database::*,
    judge_manager::*,
    judge_manager::handler::StartJudge,
    errors::{ServiceError, ServiceResult},
    statics::WAITING_QUEUE,
    utils::time::get_cur_naive_date_time,
    region::service::info::GetRegionMessage,
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;
use uuid::Uuid;
use crate::judge_server::utils::{
    filter::*,
    builder::get_judge_setting,
};
use atoi::atoi;

#[derive(Debug, Clone, Deserialize)]
struct SubmitStatusMessage {
    id: Uuid,
    owner_id: i32,
    problem_id: i32,
    problem_region: String,
    state: String,
    judge_type: String,
    setting_data: String,
    language: String,
}

impl Message for SubmitStatusMessage {
    type Result = Result<(), String>;
}

impl Handler<SubmitStatusMessage> for DbExecutor {
    type Result = Result<(), String>;
    
    fn handle(&mut self, msg: SubmitStatusMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::status::dsl::*;
        use crate::status::model::InsertableStatus;

        let _rows_inserted = diesel::insert_into(status)
            .values(&InsertableStatus{
                id: msg.id,
                owner_id: msg.owner_id,
                problem_id: msg.problem_id,
                problem_region: msg.problem_region,
                state: "Waiting".to_owned(),
                judge_type: msg.judge_type,
                setting_data: msg.setting_data,
                submit_time: get_cur_naive_date_time(),
                start_pend_time: None,
                finish_time: None,
                language: msg.language,
                host_name: None,
            })
            .execute(&self.0)
            .expect("Insert status failed");

        Ok(())
    }
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

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct SubmitResult {
    pub status_id: String,
}

pub async fn submit_service(
    data: web::Data<DBState>,
    judge_manager: web::Data<JMState>,
    problem_id: i32,
    problem_region: String,
    src: String,
    language: String,
    judge_type: String,
    output: bool,
    id: Identity,
) -> ServiceResult<SubmitResult> {
    if id.identity().is_none() {
        return Err(ServiceError::Unauthorized);
    }

    if !language_filter(&language) {
        return Err(ServiceError::BadRequest("Language doesn't support.".to_owned()));
    }

    if !judge_type_filter(&judge_type) {
        return Err(ServiceError::BadRequest("JudgeType doesn't support.".to_owned()));
    }

    // get region
    let db_result = data.db.send(GetRegionMessage {
        name: problem_region.clone(),
    }).await;

    let allow_judge_type = match db_result {
        Err(_) => { return Err(ServiceError::InternalServerError); },
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => { return Err(ServiceError::BadRequest(msg)); },
                Ok(region) => {
                    region.judge_type
                }
            }
        }
    };

    if allow_judge_type.is_some() {
        if allow_judge_type.clone().unwrap() != judge_type {
            let msg = format!("Region only allow judge_type \"{}\"", allow_judge_type.unwrap());
            return Err(ServiceError::BadRequest(msg));
        }
    }

    let cur_id = id.identity().unwrap();
    id.remember(cur_id.clone());

    // get setting
    let db_result = data.db.send(GetSettingMessage {
        region: problem_region.clone(),
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
                    let submittion_id = Uuid::new_v4();
                    match get_judge_setting(
                        data.clone(),
                        problem_region.clone(),
                        problem_id,
                        language.clone(),
                        src,
                        problem_setting.is_spj,
                        max_cpu_time,
                        max_memory,
                        problem_setting.opaque_output || output,
                    ).await {
                        Err(msg) => Err(ServiceError::BadRequest(msg)),
                        Ok(judge_setting) => {
                            let db_result = data.db.send(SubmitStatusMessage {
                                id: submittion_id,
                                owner_id: atoi::<i32>(cur_id.as_bytes()).unwrap(),
                                problem_id: problem_id,
                                problem_region: problem_region,
                                state: "Waiting".to_owned(),
                                judge_type: judge_type,
                                setting_data: serde_json::to_string(&judge_setting).unwrap(),
                                language: language,
                            }).await;
                            match db_result {
                                Err(_) => Err(ServiceError::InternalServerError),Ok(inner_result) => {
                                    match inner_result {
                                        Err(msg) => Err(ServiceError::BadRequest(msg)),
                                        Ok(_) => {
                                            {
                                                let mut lock = WAITING_QUEUE.write().unwrap();
                                                lock.push_back(submittion_id);
                                            }
                                            judge_manager.jm.do_send(StartJudge());
                                            Ok(SubmitResult { status_id: submittion_id.to_string() })
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                }
            }
        }
    }
}