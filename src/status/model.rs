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
}