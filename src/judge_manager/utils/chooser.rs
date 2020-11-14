use crate::statics::JUDGE_SERVER_INFOS;

pub fn choose_judge_server() -> Option<(String, String)> {
    let lock = JUDGE_SERVER_INFOS.read().unwrap();
    for (url, info) in lock.iter() {
        if !info.is_deprecated && info.task_number + 1 <= info.cpu_core * 2 {
            return Some((url.to_owned(), info.token.clone()));
        }
    }
    None
}