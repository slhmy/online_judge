use crate::{
    schema::contest_register_lists,
    database::*,
    utils::model::DeleteResult,
    errors::{ ServiceError, ServiceResult },
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;
use atoi::atoi;

impl Message for UnregisterMessage {
    type Result = Result<DeleteResult, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnregisterForm {
    pub region: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnregisterMessage {
    pub region_name: String,
    pub user_id: i32,
}

impl Handler<UnregisterMessage> for DbExecutor {
    type Result = Result<DeleteResult, String>;

    fn handle(&mut self, msg: UnregisterMessage, _: &mut Self::Context) -> Self::Result {

        match diesel::delete(contest_register_lists::table
            .filter(contest_register_lists::contest_region.eq(msg.region_name))
            .filter(contest_register_lists::user_id.eq(msg.user_id)))
            .execute(&self.0)
        {
            Err(_) => { Err("Error while unregistering.".to_owned()) },
            Ok(_) => { Ok(DeleteResult {
                result: "success".to_owned(),
            }) }
        }
    }
}

pub async fn unregister_service(
    data: web::Data<DBState>,
    msg: UnregisterForm,
    id: Identity,
) -> ServiceResult<DeleteResult> {
    let user_id = if id.identity().is_some() {
        Some(atoi::<i32>(id.identity().unwrap().as_bytes()).unwrap())
    } else {
        return Err(ServiceError::Unauthorized);
    };

    let db_result = data.db.send(UnregisterMessage {
        region_name: msg.clone().region,
        user_id: user_id.unwrap()
    }).await;

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