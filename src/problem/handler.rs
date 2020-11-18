use crate::{
    database::*,
    problem::service::{
        catalog::GetProblemCatalogMessage,
        catalog::get_problem_catalog_service,
        content::GetProblemMessage,
        content::get_problem as get_problem_service,
        new::{ new_problem_service, NewProblemMessage },
        update::{ update_problem_service, UpdateProblemMessage }
    },
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

pub async fn get_problem_catalog(
    data: web::Data<DBState>, 
    form: web::Form<GetProblemCatalogMessage>,
    _id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_problem_catalog_service(data, form.region.clone(), form.problems_per_page).await
        .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn get_problem(
    data: web::Data<DBState>,
    form: web::Form<GetProblemMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_problem_service(data, form.id, form.region.clone(), id).await
        .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn new_problem(
    data: web::Data<DBState>,
    form: web::Form<NewProblemMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    new_problem_service(data, form.to_owned(), id).await
        .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn update_problem(
    data: web::Data<DBState>,
    form: web::Form<UpdateProblemMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    update_problem_service(data, form.to_owned(), id).await
        .map(|res| HttpResponse::Ok().json(&res))
}