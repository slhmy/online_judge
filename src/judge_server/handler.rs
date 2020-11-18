use crate::{
    database::*,
    judge_manager::*,
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;
use actix_web::Error;
use futures::StreamExt;

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

pub async fn get_file(mut body: web::Payload) -> Result<HttpResponse, Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }
    let mut file = File::create("foo.zip")?;
    file.write_all(&bytes)?;

    Ok(HttpResponse::Ok().finish())
}