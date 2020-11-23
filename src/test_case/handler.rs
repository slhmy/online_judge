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
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

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
    mut payload: Multipart,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    let mut bytes = web::BytesMut::new();
    // iterate over multipart stream
    let mut filename = None;
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        if filename.is_none() {
            filename = Some(content_type.get_filename().unwrap().to_owned());
        } else {
            // only accept one file
            if filename.clone().unwrap() != content_type.get_filename().unwrap() { break; }
        }

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            bytes.extend_from_slice(&data);
        }
    }
    new_test_case_service(data, &bytes, info.test_case_name.clone(), info.is_spj, id)
    .await
    .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn update_test_case(
    data: web::Data<DBState>,
    info: web::Path<UploadTestCaseInfo>,
    mut payload: Multipart,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    let mut bytes = web::BytesMut::new();
    // iterate over multipart stream
    let mut filename = None;
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        if filename.is_none() {
            filename = Some(content_type.get_filename().unwrap().to_owned());
        } else {
            // only accept one file
            if filename.clone().unwrap() != content_type.get_filename().unwrap() { break; }
        }

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            bytes.extend_from_slice(&data);
        }
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