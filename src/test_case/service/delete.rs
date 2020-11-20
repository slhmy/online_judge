use crate::{
    schema::test_cases,
    database::*,
    test_case::model::DeleteTestCaseResult,
    errors::{ ServiceError, ServiceResult },
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;
use std::fs;

impl Message for DeleteTestCaseMessage {
    type Result = Result<DeleteTestCaseResult, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTestCaseMessage {
    pub name: String,
}

impl Handler<DeleteTestCaseMessage> for DbExecutor {
    type Result = Result<DeleteTestCaseResult, String>;

    fn handle(&mut self, msg: DeleteTestCaseMessage, _: &mut Self::Context) -> Self::Result {
        let path = "data/test_case/".to_owned() + &msg.name;
        match diesel::delete(test_cases::table
            .filter(test_cases::name.eq(msg.name)))
            .execute(&self.0) 
        {
            Err(_) => { Err("Error while deleting test_case.".to_owned()) },
            Ok(_) => {
                match fs::remove_dir_all(&path)
                {
                    Err(_) => { Err("Error while deleting test_case related folder.".to_owned()) },
                    Ok(_) => { Ok(DeleteTestCaseResult {
                        result: "success".to_owned(),
                    }) }
                }
            }
        }
    }
}

pub async fn delete_test_case_service(
    data: web::Data<DBState>,
    msg: DeleteTestCaseMessage,
    _id: Identity,
) -> ServiceResult<DeleteTestCaseResult> {
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