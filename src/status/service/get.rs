use crate::{
    database::*,
    errors::{ServiceError, ServiceResult},
    status::model::*,
    status::utils::mapper::*,
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use uuid::Uuid;
use actix_identity::Identity;

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct OwnerPreview {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct ProblemPreview {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct DetailedStatus {
    pub judge_result: Option<MappedJudgeResult>,
    pub err_result: Option<ErrResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetStatusMessage {
    pub id: Uuid,
}

impl Message for GetStatusMessage {
    type Result = Result<DetailedStatus, String>;
}

impl Handler<GetStatusMessage> for DbExecutor {
    type Result = Result<DetailedStatus, String>;
    
    fn handle(&mut self, msg: GetStatusMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::status;

        let status = status::table
            .filter(status::id.eq(msg.id))
            .first::<Status>(&self.0)
            .expect("Error loading status.");

            let mut judge_result: Option<JudgeResult> = None;
            let mut err_result: Option<ErrResult> = None;

        if status.result_data.is_some() {
            let result_str = status.result_data.unwrap().clone();
            let err_checker: ErrChecker = serde_json::from_str(&result_str).unwrap();
            if err_checker.err.is_none() {
                judge_result = Some(serde_json::from_str(&result_str).unwrap());
            } else {
                err_result = Some(serde_json::from_str(&result_str).unwrap());
            }
        }
        
        Ok(DetailedStatus{
            judge_result: if judge_result.is_none() { None } else {
                let inner_result = judge_result.unwrap();
                let mut final_output = MappedJudgeResult {
                    err: inner_result.err,
                    data: Vec::new(),
                };
                for data in inner_result.data.iter() {
                    final_output.data.push(MappedJudgeResultData {
                        cpu_time: data.cpu_time,
                        real_time: data.real_time,
                        memory: data.memory,
                        signal: data.signal,
                        exit_code: data.exit_code,
                        error: err_mapper(data.error),
                        result: result_mapper(data.result),
                        test_case: data.test_case.clone(),
                        output_md5: data.output_md5.clone(),
                        output: data.output.clone(),
                    });
                }
                Some(final_output)
            },
            err_result: err_result,
        })
    }
}

pub async fn get_status_service(
    data: web::Data<DBState>,
    msg: GetStatusMessage,
    _id: Identity,
) -> ServiceResult<DetailedStatus> {
    let db_result = data.db.send(msg).await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(result) => Ok(result),
            }
        }
    }
}