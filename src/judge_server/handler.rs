use crate::{
    database::*,
    judge_manager::*,
    errors::ServiceError,
};
use actix_web::{ HttpResponse, web, Error };
use actix_identity::Identity;
use std::io::Write;

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

use super::service::{
    info::server_info,
    submit::submit_service,
};

pub async fn get_server_info(id: Identity) -> Result<HttpResponse, ServiceError> {
    server_info(id).await.map(|res| HttpResponse::Ok().json(&res))
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitRequestForm {
    pub problem_id: i32,
    pub problem_region: String,
    pub src: String,
    pub language: String,
    pub judge_type: String,
    pub output: bool,
}

pub async fn submit(
    data: web::Data<DBState>,
    judge_manager: web::Data<JMState>,
    form: web::Form<SubmitRequestForm>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    submit_service(
        data,
        judge_manager,
        form.problem_id,
        form.problem_region.clone(),
        form.src.clone(),
        form.language.clone(),
        form.judge_type.clone(),
        form.output,
        id
    ).await.map(|res| HttpResponse::Ok().json(&res))
}

pub async fn get_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("data/tmp/{}", sanitize_filename::sanitize(&filename));

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}