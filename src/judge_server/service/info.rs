use crate::{
    errors::ServiceResult,
    judge_server::model::JudgeServerInfo,
};
use actix_identity::Identity;
use crate::JUDGE_SERVER_INFOS;

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct OutJudgeServerInfo {
    pub judger_version: String,
    pub hostname: String,
    pub cpu_core: i32,
    pub memory: f64,
    pub cpu: f64,
    pub service_url: String,
    pub last_heartbeat: i32,
}

impl From<JudgeServerInfo> for OutJudgeServerInfo {
    fn from(info: JudgeServerInfo) -> Self {
        let JudgeServerInfo {
            judger_version,
            hostname,
            cpu_core,
            memory,
            cpu,
            service_url,
            token: _,
            heartbeat_time,
        } = info;

        let memory = memory as f64;
        let cpu = cpu as f64;
        let last_heartbeat = heartbeat_time.elapsed().unwrap().as_secs() as i32;

        Self {
            judger_version,
            hostname,
            cpu_core,
            memory,
            cpu,
            service_url,
            last_heartbeat,
        }
    }
}

pub async fn server_info(
    _id: Identity,
) -> ServiceResult<Vec<OutJudgeServerInfo>> {
    let lock = JUDGE_SERVER_INFOS.read().unwrap();
    let mut info_vec: Vec<OutJudgeServerInfo> = Vec::new();
    for (_url, info) in lock.iter() {
        info_vec.push(OutJudgeServerInfo::from(info.clone()));
    }
    Ok(info_vec)
}