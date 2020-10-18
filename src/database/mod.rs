use diesel::prelude::*;
use actix::prelude::*;
use dotenv::dotenv;
use std::env;

pub struct DbExecutor(pub PgConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

/// This is state where we will store *DbExecutor* address.
pub struct State {
    pub db: Addr<DbExecutor>,
}

pub fn create_db_executor() -> Addr<DbExecutor> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SyncArbiter::start(3, move || {
        DbExecutor(PgConnection::establish(&database_url).unwrap())
    })
}