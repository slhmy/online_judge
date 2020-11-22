use crate::{
    database::*,
    errors::{ServiceError, ServiceResult},
    contest::service::catalog::ContestCatalogElement,
    contest::model::{ Contest },
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;
use atoi::atoi;

#[derive(Debug, Clone, Deserialize)]
pub struct GetContestForm {
    pub region: String,
}

impl Message for GetContestMessage {
    type Result = Result<ContestCatalogElement, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetContestMessage {
    pub user_id: Option<i32>,
    pub region: String,
}

impl Handler<GetContestMessage> for DbExecutor {
    type Result = Result<ContestCatalogElement, String>;

    fn handle(&mut self, msg: GetContestMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::contests::dsl::*;
        use crate::schema::contest_register_lists;
        use diesel::dsl::*;

        let result = contests.filter(region.eq(&msg.region))
            .first::<Contest>(&self.0);

        match result {
            Err(_) => { Err("Error while creating new contest.".to_owned()) },
            Ok(contest) => {
                let is_registered = if msg.user_id.is_some() {
                    match contest_register_lists::table
                        .filter(contest_register_lists::user_id.eq(msg.user_id.unwrap()))
                        .filter(contest_register_lists::contest_region.eq(contest.region.clone()))
                        .select(count_star())
                        .first::<i64>(&self.0) {
                        Err(_) => false,
                        Ok(count) => { if count >= 1 { true } else { false } },
                    }
                } else { false };
                Ok(ContestCatalogElement{
                    region: contest.region,
                    name: contest.name,
                    state: contest.state,
                    start_time: contest.start_time,
                    end_time: contest.end_time,
                    seal_before_end: contest.seal_before_end,
                    register_end_time: contest.register_end_time,
                    is_registered: is_registered,
                }) 
            }
        }
    }
}

pub async fn get_contest_service (
    data: web::Data<DBState>,
    msg: GetContestForm,
    id: Identity,
) -> ServiceResult<ContestCatalogElement> {

    let user_id = if id.identity().is_some() {
        Some(atoi::<i32>(id.identity().unwrap().as_bytes()).unwrap())
    } else { None };

    let db_result = data.db.send(GetContestMessage {
        user_id: user_id,
        region: msg.region,
    }).await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(catalog) => Ok(catalog),
            }
        }
    }
}