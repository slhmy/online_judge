use crate::{
    database::*,
    region::service::{
        tags::{ get_region_tags_service, GetRegionTagsMessage },
        info::{ get_region_service, GetRegionMessage},
    },
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

pub async fn get_info(
    data: web::Data<DBState>, 
    form: web::Form<GetRegionMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_region_service(data, form.to_owned(), id).await
        .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn get_tags(
    data: web::Data<DBState>, 
    form: web::Form<GetRegionTagsMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_region_tags_service(data, form.to_owned(), id).await
        .map(|res| HttpResponse::Ok().json(&res))
}