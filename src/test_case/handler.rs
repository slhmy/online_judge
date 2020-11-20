use crate::{
    database::*,
    test_case::service::{
        catalog::GetTestCaseCatalogMessage,
        catalog::get_test_case_catalog_service,
        new::new_test_case_service,
        update::update_test_case_service,
        delete::{ delete_test_case_service, DeleteTestCaseMessage },
    },
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;
use futures::StreamExt;

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

#[derive(Deserialize)]
pub struct UploadTestCaseInfo {
    pub test_case_name: String,
    pub is_spj: bool,
}

pub async fn new_test_case(
    data: web::Data<DBState>,
    info: web::Path<UploadTestCaseInfo>,
    mut body: web::Payload,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = match item {
            Ok(item) => item,
            Err(_) => { return Err(ServiceError::BadRequest("Error while getting file.".to_owned())); },
        };
        bytes.extend_from_slice(&item);
    }
    new_test_case_service(data, &bytes, info.test_case_name.clone(), info.is_spj, id)
    .await
    .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn update_test_case(
    data: web::Data<DBState>,
    info: web::Path<UploadTestCaseInfo>,
    mut body: web::Payload,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = match item {
            Ok(item) => item,
            Err(_) => { return Err(ServiceError::BadRequest("Error while getting file.".to_owned())); },
        };
        bytes.extend_from_slice(&item);
    }
    update_test_case_service(data, &bytes, info.test_case_name.clone(), info.is_spj, id)
    .await
    .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn delete_test_case(
    data: web::Data<DBState>,
    form: web::Form<DeleteTestCaseMessage>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    delete_test_case_service(data, form.to_owned(), id).await
        .map(|res| HttpResponse::Ok().json(&res))
}