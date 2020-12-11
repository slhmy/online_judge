use super::config::*;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone)]
pub struct JudgeServerInfo {
    pub judger_version: String,
    pub hostname: String,
    pub cpu_core: i32,
    pub memory: f32,
    pub cpu: f32,
    pub task_number: i32,
    pub service_url: String,
    pub token: String,
    pub heartbeat_time: SystemTime,
    pub is_deprecated: bool,
}