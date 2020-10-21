use crate::{
    *,
    database::*,
    user::model::*,
    encryption::encode::make_hash,
    utils::regex_matcher::RegexMatcher,
};
use actix::prelude::*;
use actix_web::{HttpResponse, web};
use actix_identity::Identity;
use diesel::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginMessage {
    pub identity_info: String,
    pub password: String,
}

impl Message for LoginMessage {
    type Result = Result<OutUser, String>;
}

impl Handler<LoginMessage> for DbExecutor {
    type Result = Result<OutUser, String>;

    fn handle(&mut self, msg: LoginMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let operation_result = 
            if msg.identity_info.is_email() {
                users.filter(email.eq(msg.identity_info)).limit(1).load::<User>(&self.0)
            } else if msg.identity_info.is_mobile() {
                users.filter(mobile.eq(msg.identity_info)).limit(1).load::<User>(&self.0)
            } else {
                users.filter(username.eq(msg.identity_info)).limit(1).load::<User>(&self.0)
            }
            .expect("Error loading user")
            .pop();

        if !operation_result.is_none() {
            let user = operation_result.unwrap();
            
            if make_hash(&msg.password, &user.salt) == user.hash.as_ref() { 
                Ok(OutUser::from(user)) 
            } else { 
                Err("Wrong password.".to_owned()) 
            }
        } else {
            Err("Database operate failed.\nReason: can't find your account.".to_owned())
        }
    }
}

pub async fn login(
    data: web::Data<State>, 
    form: web::Form<LoginMessage>,
    id: Identity,
) -> HttpResponse {
    // Send message to `DbExecutor` actor
    let res = data.db
        .send(form.to_owned())
        .await;
    
    match res {
        Err(_) => HttpResponse::InternalServerError().body("Unexpected Database error."),
        Ok(handler_result) => {
            match handler_result {
                Err(msg) => HttpResponse::BadRequest().body(format!("Login failed.\n{}.", msg)),
                Ok(user) => {
                    let user_id = user.id.to_string();
                    id.remember(user_id.clone());
                    HttpResponse::Ok().body(format!("Welcome {}.", user.username))
                }
            }
        }
    }
}

pub async fn logout(
    id: Identity,
) -> HttpResponse {
    if !id.identity().is_none() {
        id.forget();
        HttpResponse::Ok().body("logout successfully.")
    } else {
        HttpResponse::Ok().body("You are not online now.")
    }
}