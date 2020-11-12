use crate::judge_server::model::{
    JudgeSetting,
};
use crate::judge_server::config::*;
use std::fs::File;
use std::io::prelude::*;

pub fn get_judge_setting(
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
    let test_case = region + "_" + &problem_id.to_string();
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