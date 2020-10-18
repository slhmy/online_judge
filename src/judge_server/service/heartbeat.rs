use actix_web::{
    Responder,
    HttpResponse, 
};

pub async fn handle_heartbeat() -> impl Responder {
    println!("recieved judge_server heartbeat");
    HttpResponse::Ok()
}   