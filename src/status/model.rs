use chrono::*;
use uuid::Uuid;
use crate::schema::status;

#[derive(Debug, Clone, Deserialize, Queryable)]
pub struct Status {
    pub id: Uuid,
    pub owner_id: i32,
    pub problem_id: i32,
    pub problem_region: String,
    pub state: String,
    pub judge_type: String,
    pub result: Option<String>,
    pub score: Option<f64>,
    pub setting_data: String,
    pub result_data: Option<String>,
    pub err_reason: Option<String>,
    pub submit_time: NaiveDateTime,
    pub start_pend_time: Option<NaiveDateTime>,
    pub finish_time: Option<NaiveDateTime>,
    pub language: String,
    pub host_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Insertable, Queryable)]
#[table_name = "status"]
pub struct InsertableStatus {
    pub id: Uuid,
    pub owner_id: i32,
    pub problem_id: i32,
    pub problem_region: String,
    pub state: String,
    pub judge_type: String,
    pub setting_data: String,
    pub submit_time: NaiveDateTime,
    pub start_pend_time: Option<NaiveDateTime>,
    pub finish_time: Option<NaiveDateTime>,
    pub language: String,
    pub host_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeResult {
    pub err: Option<String>,
    pub data: Vec<JudgeResultData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct MappedJudgeResultData {
    pub cpu_time: i32,
    pub real_time: i32,
    pub memory: i32,
    pub signal: i32,
    pub exit_code: i32,
    pub error: String,
    pub result: String,
    pub test_case: String,
    pub output_md5: Option<String>,
    pub output: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct MappedJudgeResult {
    pub err: Option<String>,
    pub data: Vec<MappedJudgeResultData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct ErrResult {
    pub err: Option<String>,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrChecker {
    pub err: Option<String>,
}