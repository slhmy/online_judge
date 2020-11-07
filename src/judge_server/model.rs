use super::config::*;

#[derive(Debug, Clone, Serialize)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct JudgeSetting {
    pub language_config: LanguageConfig,
    pub src: String,
    pub max_cpu_time: i32,
    pub max_memory: i32,
    pub test_case_id: Option<String>,
    pub test_case: Option<Vec<TestCase>>,
    pub spj_version: Option<String>,
    pub spj_config: Option<SpjConfig>,
    pub spj_compile_config: Option<SpjCompileConfig>,
    pub spj_src: Option<String>,
    pub output: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JudgeResultData {
    pub cpu_time: i32,
    pub real_time: i32,
    pub memory: i32,
    pub signal: i32,
    pub exit_code: i32,
    pub error: i32,
    pub result: i32,
    pub test_case: String,
    pub output_md5: Option<String>,
    pub output: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrChecker {
    pub err: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JudgeResult {
    pub err: Option<String>,
    pub data: Vec<JudgeResultData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrResult {
    pub err: Option<String>,
    pub data: String,
}