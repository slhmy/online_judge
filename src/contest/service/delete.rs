use crate::{
    schema::contests,
    region::service::delete::DeleteRegionMessage,
    database::*,
    utils::model::DeleteResult,
    errors::{ ServiceError, ServiceResult },
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;

impl Message for DeleteContestMessage {
    type Result = Result<DeleteResult, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteContestMessage {
    pub region_name: String,
}

impl Handler<DeleteContestMessage> for DbExecutor {
    type Result = Result<DeleteResult, String>;

    fn handle(&mut self, msg: DeleteContestMessage, _: &mut Self::Context) -> Self::Result {

        match diesel::delete(contests::table
            .filter(contests::region.eq(msg.region_name)))
            .execute(&self.0)
        {
            Err(_) => { Err("Error while deleting contest.".to_owned()) },
            Ok(_) => { Ok(DeleteResult {
                result: "success".to_owned(),
            }) }
        }
    }
}

pub async fn delete_contest_service(
    data: web::Data<DBState>,
    msg: DeleteContestMessage,
    _id: Identity,
) -> ServiceResult<DeleteResult> {
    let db_result = data.db.send(DeleteRegionMessage {
        name: msg.clone().region_name,
    }).await;

    match db_result {
        Err(_) => return Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => return Err(ServiceError::BadRequest(msg)),
                Ok(_) => { },
            }
        }
    }

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