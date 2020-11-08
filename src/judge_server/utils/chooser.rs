use crate::JUDGE_SERVER_INFOS;

pub fn choose_judge_server() -> Option<(String, String)> {
    let lock = JUDGE_SERVER_INFOS.read().unwrap();
    for (url, info) in lock.iter() {
        return Some((url.to_owned(), info.token.clone()));
    }
    None
}