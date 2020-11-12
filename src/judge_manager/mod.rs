pub mod handler;
pub mod utils;
pub mod model;

use diesel::prelude::*;
use actix::prelude::*;
use crate::DATABASE_URL;

pub struct JudgeManager(pub PgConnection);

impl Actor for JudgeManager {
    type Context = SyncContext<Self>;
}

/// This is state where we will store *JudgeManager* address.
pub struct JMState {
    pub jm: Addr<JudgeManager>,
}

pub fn create_judge_manager() -> Addr<JudgeManager> {
    let database_url = (*DATABASE_URL).clone();

    SyncArbiter::start(8, move || {
        JudgeManager(PgConnection::establish(&database_url).unwrap())
    })
}