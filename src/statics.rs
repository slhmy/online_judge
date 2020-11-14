use std::{
    sync::RwLock,
    collections::{ BTreeMap, HashMap, VecDeque },
    time::SystemTime,
};
use crate::judge_server::model::JudgeServerInfo;
use regex::Regex;
use uuid::Uuid;
use dotenv::dotenv;
use std::env;

lazy_static! {
    pub static ref  WAITING_QUEUE: RwLock<VecDeque::<Uuid>> = RwLock::new(VecDeque::new());
    pub static ref ACCESS_KEY_ID: String = {
        dotenv().ok();
        env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID must be set")
    };
    pub static ref ACCESS_SECRET: String = {
        dotenv().ok();
        env::var("ACCESS_SECRET").expect("ACCESS_SECRET must be set")
    };
    pub static ref DATABASE_URL: String = {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")  
    };
    pub static ref JUDGE_SERVER_INFOS: RwLock<HashMap<String, JudgeServerInfo>> = RwLock::new(HashMap::new());
    pub static ref VERIFICATION_MAP: RwLock<BTreeMap<String, (String, SystemTime)>> = RwLock::new(BTreeMap::new());
    pub static ref RE_EMAIL: Regex = Regex::new(r"^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();
    pub static ref RE_MOBILE: Regex = Regex::new(r"^((13[0-9])|(14[5|7])|(15([0-3]|[5-9]))|(18[0,5-9]))\d{8}$").unwrap();
    pub static ref RE_PASSWORD: Regex = Regex::new(r"^\S{6,20}$").unwrap();
}