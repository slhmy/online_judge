use crate::{
    database::*,
    problem::service::{
        catalog::GetCatalogMessage,
        catalog::get_catalog as get_catalog_service,
        content::GetProblemMessage,
        content::get_problem as get_problem_service,
    },
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

pub async fn get_catalog(
    data: web::Data<State>, 
    form: web::Form<GetCatalogMessage>,
    _id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_catalog_service(data, form.region.clone(), form.problems_per_page).await
        .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn get_problem(
    data: web::Data<State>,
    form: web::Form<GetProblemMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_problem_service(data, form.id, form.region.clone(), id).await
        .map(|res| HttpResponse::Ok().json(&res))
}