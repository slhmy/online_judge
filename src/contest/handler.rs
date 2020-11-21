use crate::{
    database::*,
    region::service::new::NewRegionMessage,
    contest::service::{ 
        new::{ new_contest_service, NewContestMessage },
        register::register_service,
        delete::{ delete_contest_service, DeleteContestMessage },
    },
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

#[derive(Debug, Clone, Deserialize)]
pub struct NewContestRequest {
    pub region: String,
    pub name: String, 
    pub start_time: String,
    pub end_time: String,
    pub seal_before_end: Option<i32>,
    pub register_end_time: Option<String>,
    pub judge_type: String,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    pub contest_region: String,
    pub is_unrated: bool,
    pub password: Option<String>,
}

pub async fn new_contest(
    data: web::Data<DBState>, 
    form: web::Form<NewContestRequest>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    new_contest_service(
        data,
        NewRegionMessage {
            name: form.region.clone(),
            password: form.password.clone(),
            self_type: "contest".to_owned(),
            judge_type: form.judge_type.clone(),
        },
        NewContestMessage {
            region: form.region.clone(),
            name: form.name.clone(), 
            start_time: form.start_time.clone(),
            end_time: form.end_time.clone(),
            seal_before_end: form.seal_before_end,
            register_end_time: form.register_end_time.clone(),
        },
        id,
    ).await
    .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn delete_contest(
    data: web::Data<DBState>, 
    form: web::Form<DeleteContestMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    delete_contest_service(
        data,
        form.to_owned(),
        id,
    ).await
    .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn register(
    data: web::Data<DBState>, 
    form: web::Form<RegisterRequest>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    register_service(
        data,
        form.contest_region.clone(),
        form.is_unrated,
        form.password.clone(),
        id,
    ).await
    .map(|res| HttpResponse::Ok().json(&res))
}