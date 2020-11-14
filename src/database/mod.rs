use diesel::prelude::*;
use actix::prelude::*;
use crate::statics::DATABASE_URL;

pub struct DbExecutor(pub PgConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

/// This is state where we will store *DbExecutor* address.
pub struct DBState {
    pub db: Addr<DbExecutor>,
}

pub fn create_db_executor() -> Addr<DbExecutor> {
    let database_url = (*DATABASE_URL).clone();

    SyncArbiter::start(3, move || {
        DbExecutor(PgConnection::establish(&database_url).unwrap())
    })
}