use super::mapper::*;
use crate::judge_manager::model::*;

#[derive(Debug, Clone, Serialize)]
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

pub fn get_judge_result(judge_type: String, result_str: String) -> (Option<String>, Option<f64>, Option<String>) {
    let err_checker: ErrChecker = serde_json::from_str(&result_str).unwrap();
    if err_checker.err.is_none() {
        let judge_result: JudgeResult = serde_json::from_str(&result_str).unwrap();
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
            "OI" => { (None, Some(100.0 * (passed_test_cases as f64 / total_test_cases as f64)), None) },
            _ => { (Some(final_result), None, None) },
        }
    } else {
        let err_result: ErrResult = serde_json::from_str(&result_str).unwrap();
        (
            Some(err_result.err.unwrap()),
            None,
            Some(err_result.data),
        )
    }
}