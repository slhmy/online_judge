use actix_web::{
    web,
    Responder,
    HttpRequest,
    HttpResponse, 
};
use crate::statics::JUDGE_SERVER_INFOS;
use crate::judge_server::model::JudgeServerInfo;
use std::time::SystemTime;
use crate::judge_manager::*;
use crate::judge_manager::handler::StartJudge;
use actix_web::client::Client;

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
    judge_manager: web::Data<JMState>,
    req: HttpRequest, 
    info: web::Json<HeartbeatResquest>
) -> impl Responder {
    let token = req.headers().get("x-judge-server-token").unwrap()
        .to_str().unwrap().to_string();

    if !info.service_url.is_none()
    {        
        let service_url = info.service_url.clone().unwrap();
        let (mut is_deprecated, task_number) = 
        {
            let lock = JUDGE_SERVER_INFOS.read().unwrap();
            if lock.get(&service_url).is_none() { (false, 0) } 
            else { 
                let target = lock.get(&service_url).unwrap();
                (target.is_deprecated, target.task_number)
            }
        };

        let response = Client::new()
            .post(format!("{}/ping", service_url))
            .set_header("X-Judge-Server-Token", token.clone())
            .set_header("Content-Type", "application/json")
            .send()
            .await;

        if !response.is_ok() {
            is_deprecated = true;
            info!("setting is_deprecated to true");
        } else {
            is_deprecated = false;
        }

        let now = SystemTime::now();
        let judge_server_info = JudgeServerInfo {
            judger_version: info.judger_version.clone(),
            hostname: info.hostname.clone(),
            cpu_core: info.cpu_core,
            memory: info.memory,
            cpu: info.cpu,
            task_number: task_number,
            service_url: service_url,
            token: token.clone(),
            heartbeat_time: now,
            is_deprecated: is_deprecated,
        };
        let mut lock = JUDGE_SERVER_INFOS.write().unwrap();
        lock.insert(info.service_url.clone().unwrap(), judge_server_info);

        if !is_deprecated {
            judge_manager.jm.do_send(StartJudge());
        }
    }
   
    HttpResponse::Ok()
        .set_header("X-Judge-Server-Token", token)
        .set_header("Content-Type", "application/json")
        .json(HeartbeatResponse {
            data: "success".to_owned(),
            error: None,
        })
}   