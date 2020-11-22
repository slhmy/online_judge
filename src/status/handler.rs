use crate::{
    database::*,
    status::service::{
        catalog::{ GetStatusCatalogMessage, get_status_catalog_service },
        get::{ get_status_service, GetStatusMessage }
    },
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

pub async fn get_status_catalog(
    data: web::Data<DBState>, 
    form: web::Form<GetStatusCatalogMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_status_catalog_service(
        data, form.region.clone(), form.count_per_page, 
        form.problem_id, form.problem_title.clone(), form.user_id,
        form.username.clone(), form.language.clone(),
        form.page_number, id
    ).await
    .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn get_status(
    data: web::Data<DBState>, 
    form: web::Form<GetStatusMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_status_service(data, form.to_owned(), id).await
    .map(|res| HttpResponse::Ok().json(&res))
}