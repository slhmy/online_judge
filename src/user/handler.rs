use crate::{
    database::*,
    user::service::{
        catalog::{ get_user_catalog_service, GetUserCatalogMessage }
    },
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

pub async fn get_user_catalog(
    data: web::Data<DBState>, 
    form: web::Form<GetUserCatalogMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_user_catalog_service(data, form.to_owned(), id).await
        .map(|res| HttpResponse::Ok().json(&res))
}