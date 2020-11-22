use crate::{
    schema::contests,
    database::*,
    contest::model::{ Contest, OutContest },
    errors::{ ServiceError, ServiceResult },
    region::service::{ info::GetRegionMessage, new::NewRegionMessage },
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;
use chrono::*;

#[derive(Debug, Clone, Deserialize, Insertable, Queryable)]
#[table_name = "contests"]
struct InsertableContest {
    region: String,
    name: String,
    state: String, 
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    seal_before_end: Option<i32>,
    register_end_time: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewContestForm {
    pub region: String,
    pub name: String, 
    pub start_time: String,
    pub end_time: String,
    pub seal_before_end: Option<i32>,
    pub register_end_time: Option<String>,
    pub judge_type: String,
    pub password: Option<String>,
}

impl Message for NewContestMessage {
    type Result = Result<OutContest, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewContestMessage {
    pub region: String,
    pub name: String, 
    pub start_time: String,
    pub end_time: String,
    pub seal_before_end: Option<i32>,
    pub register_end_time: Option<String>,
}

impl Handler<NewContestMessage> for DbExecutor {
    type Result = Result<OutContest, String>;

    fn handle(&mut self, msg: NewContestMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::contests::dsl::contests;

        let start_time = 
            match Utc.datetime_from_str(&msg.start_time, "%Y-%m-%d %H:%M:%S") {
                Ok(time) => { NaiveDate::from_ymd(time.year(), time.month(), time.day())
                    .and_hms(time.hour(), time.minute(), time.second()) },
                Err(_) => { return Err("Time format is not correct".to_owned()); },
            };
        
        let end_time = 
            match Utc.datetime_from_str(&msg.end_time, "%Y-%m-%d %H:%M:%S") {
                Ok(time) => { NaiveDate::from_ymd(time.year(), time.month(), time.day())
                    .and_hms(time.hour(), time.minute(), time.second()) },
                Err(_) => { return Err("Time format is not correct".to_owned()); },
            };
        
        let register_end_time = if msg.register_end_time.is_none() {
            end_time
        } else { 
            match Utc.datetime_from_str(&(msg.register_end_time.unwrap()), "%Y-%m-%d %H:%M:%S") {
                Ok(time) => { 
                    NaiveDate::from_ymd(time.year(), time.month(), time.day())
                    .and_hms(time.hour(), time.minute(), time.second()) },
                Err(_) => { return Err("Time format is not correct".to_owned()); },
            }
        };

        let state = "Preparing".to_string();
        let result = diesel::insert_into(contests)
            .values(&InsertableContest{
                region: msg.region,
                name: msg.name,
                state: state, 
                start_time: start_time,
                end_time: end_time,
                seal_before_end: msg.seal_before_end,
                register_end_time: register_end_time,
            })
            .get_result::<Contest>(&self.0);

        match result {
            Err(_) => { Err("Error while creating new contest.".to_owned()) },
            Ok(inner_result) => { Ok(OutContest::from(inner_result)) }
        }
    }
}

pub async fn new_contest_service(
    data: web::Data<DBState>,
    region_msg: NewRegionMessage,
    contest_msg: NewContestMessage,
    _id: Identity,
) -> ServiceResult<OutContest> {

    let db_result = data.db.send(GetRegionMessage {
        name: region_msg.clone().name,
    }).await;

    match db_result {
        Err(_) => { return Err(ServiceError::InternalServerError); },
        Ok(inner_result) => {
            match inner_result {
                Ok(_) => { return Err(ServiceError::BadRequest("Region already exists.".to_owned())); },
                Err(_) => {
                    let db_result = data.db.send(region_msg.to_owned()).await;
                    match db_result {
                        Err(_) => { return Err(ServiceError::InternalServerError); },
                        Ok(inner_result) => { 
                            match inner_result {
                                Err(msg) => { return Err(ServiceError::BadRequest(msg)) },
                                Ok(_) => {},
                            }
                        }
                    }
                }
            }
        }
    }

    let db_result = data.db.send(contest_msg).await;

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