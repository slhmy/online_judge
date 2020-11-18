use crate::{
    database::*,
    test_case::service::{
        catalog::GetTestCaseCatalogMessage,
        catalog::get_test_case_catalog_service,
    },
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

pub async fn get_test_case_catalog(
    data: web::Data<DBState>, 
    form: web::Form<GetTestCaseCatalogMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    get_test_case_catalog_service(
        data, form.elements_per_page, id
    ).await
    .map(|res| HttpResponse::Ok().json(&res))
}