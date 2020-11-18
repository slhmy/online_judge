use crate::statics::JUDGE_SERVER_INFOS;

pub fn choose_judge_server() -> Option<(String, String)> {
    let lock = JUDGE_SERVER_INFOS.read().unwrap();
    for (url, info) in lock.iter() {
        let last_heartbeat = info.heartbeat_time.elapsed().unwrap().as_secs() as i32;
        if !info.is_deprecated && info.task_number + 1 <= info.cpu_core * 2 && last_heartbeat <= 5 {
            return Some((url.to_owned(), info.token.clone()));
        }
    }
    None
}