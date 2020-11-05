use actix_web::{
    Responder,
    HttpResponse, 
};
use crate::JUDGE_SERVER_TOKEN;

#[derive(Debug, Clone, Serialize)]
struct HeartbeatResponse {
    data: String,
    error: Option<String>,
}

pub async fn handle_heartbeat() -> impl Responder {
    let token = (*JUDGE_SERVER_TOKEN).clone();
   
    HttpResponse::Ok()
        .set_header("X-Judge-Server-Token", token)
        .set_header("Content-Type", "application/json")
        .json(HeartbeatResponse {
            data: "success".to_owned(),
            error: None,
        })
}   