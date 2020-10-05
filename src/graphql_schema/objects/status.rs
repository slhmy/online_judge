use juniper::{GraphQLEnum, GraphQLObject};
use super::user::*;
use super::problem::*;

#[derive(GraphQLEnum)]
/// Inner enum_type for Status.
/// It is created according to [Qingdao Judge Server Doc](https://docs.onlinejudge.me/#/judgeserver/api)
pub enum ResultType {
    WrongAnswer,
    Success,
    CpuTimeLimitExceeded,
    RealTimeLimitExceeded,
    MemoryLimitExceeded,
    RuntimeError,
    SystemError,
}

#[derive(GraphQLObject)]
// Inner object for Status.
// Every 'JudgeResult' is for one testcase.
// Most of them are given from judge server.
pub struct JudgeResult {
    pub test_case: i32,
    pub result: ResultType,
    pub cpu_time: i32,
    pub real_time: i32,
    pub memory: i32,
    pub signal: i32,
    pub exit_code: i32,
    pub error: i32,
    pub output: String,
}

#[derive(GraphQLEnum)]
/// Inner enum_type for Status.
/// This type indicates which kind of judge strategy has been chosen.
pub enum JudgeStrategy {
    OI,
    ACM,
}

#[derive(GraphQLObject)]
/// Inner object for Status.
/// Include all general datas. Most of them can be used in filters. 
pub struct StatusInformation {
    /// Shows which problem the status belongs to.
    pub problem: Problem,
    /// Shows which user submitted this status.
    pub owner: User,
    /// Set region for status.
    /// This is mainly for regional query.
    /// For example, each contest should have its own region.
    pub region: String,
    /// When is this submission occured.
    pub submit_time: String,
    /// When is the Judge finished.
    pub finish_time: String,
    pub judge_strategy: JudgeStrategy,
    /// General result for each status.
    /// For OI type, shows scores.
    /// For ACM type, shows result.
    pub final_result: String,
}

#[derive(GraphQLObject)]
/// Basic object recording result for every submission.
pub struct Status {
    /// Unique identification for each status.
    /// Decide to use self-increasing sequence so that it can improve searching performance.
    pub id: i32,
    pub information: StatusInformation,
    /// Shows if the submission can be Judged.
    pub is_compile_error: bool,
    /// If compile error happens, the compile error message is given.
    /// It can help user to locate and solve their problems.
    pub compile_error_message: Option<String>,
    /// Details for every testcase.
    /// If meets compile error, then the details are not given
    pub judge_details: Option<Vec<JudgeResult>>,
    /// Default lowest identity which is needed to visit this status.
    pub lowest_user_identity: UserIdentity,
    /// Key which admits anybody to view this status.
    /// It will be NONE until the owner generate it.
    pub special_permissions_key: Option<String>,
}