use actix_web::{
    web,
    Responder,
    HttpRequest,
    HttpResponse, 
};
use crate::JUDGE_SERVER_INFOS;
use crate::judge_server::model::JudgeServerInfo;
use std::time::SystemTime;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HeartbeatResquest {
    pub judger_version: String,
    pub hostname: String,
    pub cpu_core: i32,
    pub memory: f32,
    pub cpu: f32,
    pub service_url: Option<String>
}

#[derive(Debug, Clone, Serialize)]
struct HeartbeatResponse {
    data: String,
    error: Option<String>,
}

pub async fn handle_heartbeat(
    req: HttpRequest, 
    info: web::Json<HeartbeatResquest>
) -> impl Responder {
    let token = req.headers().get("x-judge-server-token").unwrap()
        .to_str().unwrap().to_string();

    if !info.service_url.is_none()
    {
        let service_url = info.service_url.clone().unwrap();
        let now = SystemTime::now();
        let judge_server_info = JudgeServerInfo {
            judger_version: info.judger_version.clone(),
            hostname: info.hostname.clone(),
            cpu_core: info.cpu_core,
            memory: info.memory,
            cpu: info.cpu,
            service_url: service_url,
            token: token.clone(),
            heartbeat_time: now,
        };
        let mut lock = JUDGE_SERVER_INFOS.write().unwrap();
        lock.insert(info.service_url.clone().unwrap(), judge_server_info);
    }
   
    HttpResponse::Ok()
        .set_header("X-Judge-Server-Token", token)
        .set_header("Content-Type", "application/json")
        .json(HeartbeatResponse {
            data: "success".to_owned(),
            error: None,
        })
}   