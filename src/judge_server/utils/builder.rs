use crate::judge_server::model::{
    JudgeSetting,
};
use crate::judge_server::config::*;
use std::fs::File;
use std::io::prelude::*;
use actix::prelude::*;
use diesel::prelude::*;
use crate::database::*;
use actix_web::web;

#[derive(Debug, Clone, Deserialize)]
pub struct GetTestCaseName {
    pub id: i32,
    pub region: String,
}

impl Message for GetTestCaseName {
    type Result = Result<Option<String>, String>;
}

impl Handler<GetTestCaseName> for DbExecutor {
    type Result = Result<Option<String>, String>;
    
    fn handle(&mut self, msg: GetTestCaseName, _: &mut Self::Context) -> Self::Result {
        use crate::schema::problems::dsl::*;

        let result = problems.filter(region.eq(msg.region))
            .filter(id.eq(msg.id))
            .select(test_case)
            .first::<Option<String>>(&self.0)
            .expect("Error loading problems.");

        Ok(result)
    }
}

pub async fn get_judge_setting(
    data: web::Data<DBState>,
    region: String,
    problem_id: i32,
    language: String,
    src: String,
    is_spj: bool,
    max_cpu_time: i32,
    max_memory: i32,
    output: bool,
) -> Result<JudgeSetting, String> {
    let mut spj_version: Option<String> = None;
    let mut spj_config: Option<SpjConfig> = None;
    let mut spj_compile_config: Option<SpjCompileConfig> = None;
    let mut spj_src: Option<String> = None;

    let wraped_test_case = data.db.send(GetTestCaseName {
        id: problem_id,
        region: region,
    }).await.expect("Unexpected Error").expect("");

    let test_case = if wraped_test_case.is_some() {
        wraped_test_case.unwrap()
    } else {
        return Err("Problem doesn't have test cases.".to_owned());
    };

    if is_spj {
        spj_version = Some("1".to_owned());
        spj_config = Some(c_lang_spj_config());
        spj_compile_config = Some(c_lang_spj_compile());

        let mut file = File::open("data/test_case/".to_owned() + &test_case +"/spj_src.c").expect("Error opening spj src");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Error reading spj src");
        spj_src = Some(contents);
    }
    
    let judge_setting = JudgeSetting {
        language_config: get_lang_config(&language),
        src: src,
        max_cpu_time: max_cpu_time,
        max_memory: max_memory,
        test_case_id: Some(test_case),
        test_case: None,
        spj_version: spj_version,
        spj_config: spj_config,
        spj_compile_config: spj_compile_config,
        spj_src: spj_src,
        output: output,
    };

    Ok(judge_setting)
}