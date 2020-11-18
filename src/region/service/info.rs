use crate::{
    database::*,
    region::model::{ Region, OutRegion },
    errors::{ServiceError, ServiceResult},
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;

impl Message for GetRegionMessage {
    type Result = Result<OutRegion, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRegionMessage {
    pub name: String,
}

impl Handler<GetRegionMessage> for DbExecutor {
    type Result = Result<OutRegion, String>;

    fn handle(&mut self, msg: GetRegionMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::regions::dsl::*;

        let result = regions
            .filter(name.eq(msg.name))
            .first::<Region>(&self.0);

        match result {
            Err(_) => { Err("Error loading region".to_owned()) },
            Ok(inner_result) => { Ok(OutRegion::from(inner_result)) }
        }
    }
}

pub async fn _get_region_service(
    data: web::Data<DBState>,
    msg: GetRegionMessage,
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