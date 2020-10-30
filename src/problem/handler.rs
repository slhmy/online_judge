use crate::{
    database::*,
    problem::service::catalog::{
        get_catalog as get_catalog_service,
        GetCatalogMessage,
    },
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

pub async fn get_catalog(
    data: web::Data<State>, 
    form: web::Form<GetCatalogMessage>,
    _id: Identity,
) -> Result<HttpResponse, ServiceError>  {
    get_catalog_service(data, form.region.clone(), form.problems_per_page)
        .map(|res| HttpResponse::Ok().json(&res))
}