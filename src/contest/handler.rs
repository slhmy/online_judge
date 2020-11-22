use crate::{
    database::*,
    region::service::new::NewRegionMessage,
    contest::service::{ 
        new::{ new_contest_service, NewContestMessage, NewContestForm },
        register::{ register_service, RegisterForm },
        delete::{ delete_contest_service, DeleteContestMessage },
        catalog::{ get_contest_catalog_service, GetContestCatalogForm },
        get::{ get_contest_service, GetContestForm },
    },
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

pub async fn get_contest(
    data: web::Data<DBState>, 
    form: web::Form<GetContestForm>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_contest_service(
        data,
        form.to_owned(),
        id,
    ).await
    .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn new_contest(
    data: web::Data<DBState>, 
    form: web::Form<NewContestForm>,
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

pub async fn get_contest_catalog(
    data: web::Data<DBState>, 
    form: web::Form<GetContestCatalogForm>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_contest_catalog_service(
        data,
        form.to_owned(),
        id,
    ).await
    .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn register(
    data: web::Data<DBState>, 
    form: web::Form<RegisterForm>,
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