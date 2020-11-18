use crate::{
    schema::regions,
    database::*,
    region::model::{ Region, OutRegion },
    errors::{ServiceError, ServiceResult},
    utils::{
        encryption::encode::{
            make_salt,
            make_hash,
        }
    },
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;

#[derive(Debug, Clone, Deserialize, Insertable, Queryable)]
#[table_name = "regions"]
struct InsertableRegion {
    name: String,
    need_pass: bool,
    salt: Option<String>,
    hash: Option<Vec<u8>>,
    self_type: String,
    judge_type: Option<String>,
}

impl Message for NewRegionMessage {
    type Result = Result<OutRegion, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewRegionMessage {
    pub name: String,
    pub password: Option<String>,
    pub self_type: String,
    pub judge_type: String,
}

impl Handler<NewRegionMessage> for DbExecutor {
    type Result = Result<OutRegion, String>;

    fn handle(&mut self, msg: NewRegionMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::regions::dsl::regions;

        let need_pass = if msg.password.is_none() { false } else { true };
        let salt = if msg.password.is_none() { None } else { Some(make_salt()) };
        let hash = if msg.password.is_none() { None } else { Some(make_hash(&msg.password.unwrap(), &salt.clone().unwrap()).to_vec()) };
        let result = diesel::insert_into(regions)
            .values(&InsertableRegion{
                name: msg.name,
                need_pass: need_pass,
                salt: salt,
                hash: hash,
                self_type: msg.self_type,
                judge_type: Some(msg.judge_type),
            })
            .get_result::<Region>(&self.0);

        match result {
            Err(_) => { Err("Error while creating new region.".to_owned()) },
            Ok(inner_result) => { Ok(OutRegion::from(inner_result)) }
        }
    }
}

pub async fn _new_region_service(
    data: web::Data<DBState>,
    msg: NewRegionMessage,
    _id: Identity,
) -> ServiceResult<OutRegion> {
    let db_result = data.db
    .send(msg)
    .await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(region) => Ok(region),
            }
        }
    }
}