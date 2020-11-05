use actix_web::{
    Responder,
    HttpResponse, 
};
use crate::JUDGE_SERVER_TOKEN;
use actix_web::client::Client;

pub async fn ping_judge_server() -> impl Responder {
    let token = (*JUDGE_SERVER_TOKEN).clone();
   
    let response = Client::new()
        .post("http://127.0.0.1:12358/ping")
        .set_header("X-Judge-Server-Token", token)
        .set_header("Content-Type", "application/json")
        .send()
        .await;
    info!("Response: {:?}", response);
    HttpResponse::Ok().body("ping sended")
}