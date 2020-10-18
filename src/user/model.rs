use chrono::*;

#[derive(Debug, Clone, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub job_number: Option<String>,
    pub role: String,
    pub salt: String,
    pub register_time: NaiveDateTime,
    pub hash: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OutUser {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub job_number: Option<String>,
    pub register_time: NaiveDateTime,
    pub role: String,
}

impl From<User> for OutUser {
    fn from(user: User) -> Self {
        let User {
            id,
            username,
            email,
            mobile,
            job_number,
            register_time,
            role,
            ..
        } = user;

        Self {
            id,
            username,
            email,
            mobile,
            register_time,
            job_number,
            role,
        }
    }
}

use crate::{
    *,
    database::*,
};
use actix::prelude::*;
use diesel::prelude::*;

pub struct UserId(pub i32);

impl Message for UserId {
    type Result = Result<OutUser, String>;
}

impl Handler<UserId> for DbExecutor {
    type Result = Result<OutUser, String>;

    fn handle(&mut self, msg: UserId, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let operation_result: Result<User, _>  = users
            .find(msg.0)
            .get_result(&self.0);

        match operation_result {
            Err(system_msg) => Err(format!("Database operate failed.\nSystem_msg: {}", system_msg)),
            Ok(user) => Ok(OutUser::from(user)),
        }
    }
}