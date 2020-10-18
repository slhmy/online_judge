use crate::{
    database::*,
    user::model::*,
    encryption::encode::{
        make_salt,
        make_hash,
    },
    schema::users,
    utils::{
        role_filter::customize_role,
        regex_matcher::RegexMatcher,
    }
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::{HttpResponse, web, Responder};

#[derive(Debug, Clone, Deserialize, Insertable, Queryable)]
#[table_name = "users"]
struct InsertableUser {
    username: String,
    email: Option<String>,
    mobile: Option<String>,
    salt: String,
    role: String,
    job_number: Option<String>,
    pub hash:Vec<u8>,
}

impl Message for ResgisterMessage {
    type Result = Result<OutUser, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResgisterMessage {
    pub username: String,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub password: String,
    pub role: String,
    pub job_number: Option<String>,
}

impl Handler<ResgisterMessage> for DbExecutor {
    type Result = Result<OutUser, String>;

    fn handle(&mut self, msg: ResgisterMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::users;
        
        info!("Making salt...");
        let salt = make_salt();
        let register_information = InsertableUser {
            username: msg.username,
            role: customize_role(&msg.role),
            email: msg.email,
            mobile: msg.mobile,
            salt: salt.clone(),
            job_number: msg.job_number,
            hash: make_hash(&msg.password, &salt).to_vec(),
        };

        let operation_result: Result<User, _> = diesel::insert_into(users)
            .values(&register_information)
            .get_result(&self.0);

        match operation_result {
            Err(system_msg) => Err(format!("Database operate failed.\nSystem_msg: {}", system_msg)),
            Ok(user) => Ok(OutUser::from(user)),
        }
    }
}

pub async fn register(
    data: web::Data<State>, 
    form: web::Form<ResgisterMessage>
) -> impl Responder {

    if form.username.is_email() || form.username.is_mobile() {
        let msg = "Reason: username can't be a email/mobile.";
        return HttpResponse::BadRequest().body(format!("Register failed.\n{}.", msg));
    }

    if !form.password.is_password() {
        let msg = "Reason: password should be in length of 6-20";
        return HttpResponse::BadRequest().body(format!("Register failed.\n{}.", msg));
    }
    
    let res = data.db
        .send(form.into_inner())
        .await;
    
    match res {
        Err(_) => HttpResponse::InternalServerError().body("Unexpected Database error."),
        Ok(handler_result) => { 
            match handler_result {
                Err(msg) => HttpResponse::BadRequest().body(format!("Register failed.\n{}.", msg)),
                Ok(_) => HttpResponse::Ok().body("Successfully registered.")
            }
        }
    }
}