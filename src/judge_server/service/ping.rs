use actix_web::{
    Responder,
    HttpResponse, 
};
use crate::encryption::encode;
use actix_web::client::Client;

pub async fn ping_judge_server() -> impl Responder {
    let token = encode::sha256_token("YOUR_TOKEN_HERE");
   
    let response = Client::new()
        .post("http://127.0.0.1:12358/ping")
        .set_header("X-Judge-Server-Token", token)
        .set_header("Content-Type", "application/json")
        .send()
        .await;
    println!("Response: {:?}", response);
    HttpResponse::Ok().body("ping sended")
}