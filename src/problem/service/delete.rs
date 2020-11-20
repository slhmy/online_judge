use crate::{
    schema::problems,
    schema::status,
    database::*,
    problem::model::DeleteProblemResult,
    errors::{ ServiceError, ServiceResult },
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;

impl Message for DeleteProblemMessage {
    type Result = Result<DeleteProblemResult, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteProblemMessage {
    pub id: i32,
    pub region: String,
}

impl Handler<DeleteProblemMessage> for DbExecutor {
    type Result = Result<DeleteProblemResult, String>;

    fn handle(&mut self, msg: DeleteProblemMessage, _: &mut Self::Context) -> Self::Result {

        match diesel::delete(status::table
            .filter(status::problem_region.eq(msg.region.clone()))
            .filter(status::problem_id.eq(msg.id)))
            .execute(&self.0)
        {
            Err(_) => { Err("Error while deleting problem related status.".to_owned()) },
            Ok(_) => {
                match diesel::delete(problems::table
                    .filter(problems::region.eq(msg.region))
                    .filter(problems::id.eq(msg.id)))
                    .execute(&self.0)
                {
                    Err(_) => { Err("Error while deleting problem.".to_owned()) },
                    Ok(_) => { Ok(DeleteProblemResult {
                        result: "success".to_owned(),
                    }) }
                }
            }
        }
    }
}

pub async fn delete_problem_service(
    data: web::Data<DBState>,
    msg: DeleteProblemMessage,
    _id: Identity,
) -> ServiceResult<DeleteProblemResult> {
    let db_result = data.db.send(msg).await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(inner_result) => Ok(inner_result),
            }
        }
    }
}