use crate::{
    statics::VERIFICATION_MAP,
    database::*,
    user::{
        model::*,
        service::register::*,
    },
    utils::{
        encryption::encode::make_hash,
        role_filter::customize_role,
        regex_matcher::RegexMatcher,
        operation_result::OperationResult,
        sendsms_url_builder::get_url,
    }
};
use std::time::SystemTime;
use http::StatusCode;
use actix::prelude::*;
use actix_web::{HttpResponse, web};
use actix_web::client::Client;
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
            .expect("Error loading user.")
            .pop();

        if !operation_result.is_none() {
            let user = operation_result.unwrap();
            
            if make_hash(&msg.password, &user.salt) == user.hash.as_ref() { 
                Ok(OutUser::from(user)) 
            } else { 
                Err("Wrong password.".to_owned()) 
            }
        } else {
            Err("Can't find your Account.".to_owned())
        }
    }
}

pub async fn login(
    data: web::Data<DBState>, 
    form: web::Form<LoginMessage>,
    id: Identity,
) -> HttpResponse {
    // Send message to `DbExecutor` actor
    let res = data.db
        .send(form.to_owned())
        .await;
    
    match res {
        Err(_) => HttpResponse::InternalServerError().json(
            OperationResult {
                result_en: Some("unexpected error".to_owned()),
                msg_en: Some("Something went wrong in database".to_owned()),
                result_cn: None,
                msg_cn: None,
            }),
        Ok(handler_result) => {
            match handler_result {
                Err(msg) => HttpResponse::BadRequest().json(
                    OperationResult {
                        result_en: Some("rejected".to_owned()),
                        msg_en: Some(msg),
                        result_cn: None,
                        msg_cn: None,
                    }),
                Ok(user) => {
                    let user_id = user.id.to_string();
                    id.remember(user_id.clone());
                    HttpResponse::Ok().json(
                        OperationResult {
                            result_en: Some("success".to_owned()),
                            msg_en: Some(format!("Welcome {}.", user.username)),
                            result_cn: None,
                            msg_cn: None,
                        })
                }
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetVerificationCodeMessage {
    pub mobile: String,
}

pub async fn get_verification_code(
    form: web::Form<GetVerificationCodeMessage>,
) -> HttpResponse {
    let (url, code) = get_url(&form.mobile);
    let response = Client::new()
        .get(url.clone())
        .send()
        .await;
    info!("Url: {}", url);
    info!("Response: {:?}", response);
    let max_try_times = 3;
    for _ in 0..max_try_times {
        match response {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        let mut lock = VERIFICATION_MAP.write().unwrap();
                        lock.insert(form.mobile.clone(), (code, SystemTime::now()));
                        info!{"{}:{}", form.mobile.clone(), lock.get(&form.mobile.clone()).unwrap().0}
                        return HttpResponse::Ok().json(
                            OperationResult {
                                result_en: Some("success".to_owned()),
                                msg_en: Some("Verification code send successfully.".to_owned()),
                                result_cn: None,
                                msg_cn: None,
                            })
                    },
                    _ => {
                        return HttpResponse::InternalServerError().json(
                            OperationResult {
                                result_en: Some("error".to_owned()),
                                msg_en: Some("Error occured in sms server. Maybe your mobile is incorrect.".to_owned()),
                                result_cn: None,
                                msg_cn: None,
                            })
                    }
                }
            },
            Err(_) => {}
        }
    }

    HttpResponse::InternalServerError().json(
        OperationResult {
            result_en: Some("unexpected error".to_owned()),
            msg_en: Some("Error occured in sms server. Please retry.".to_owned()),
            result_cn: None,
            msg_cn: None,
        })
}

#[derive(Debug, Clone, Deserialize)]
pub struct QucikLoginMessage {
    pub mobile: String,
    pub verification_code: String,
}

pub async fn quick_login(
    data: web::Data<DBState>, 
    form: web::Form<QucikLoginMessage>,
    id: Identity,
) -> HttpResponse {
    // Send message to `DbExecutor` actor
    let has_verification_info = { !VERIFICATION_MAP.read().unwrap().get(&form.mobile).is_none() };
    if has_verification_info {
        let (real_verification_code, request_time) = { VERIFICATION_MAP.read().unwrap().get(&form.mobile).unwrap().clone() };
        if real_verification_code == form.verification_code {
            if request_time.elapsed().unwrap().as_secs() < 9000 {
                let res = data.db
                    .send(UserMobile(form.mobile.clone()))
                    .await;
                let user;
                match res {
                    Err(_) => { 
                        return HttpResponse::InternalServerError().json(
                            OperationResult {
                                result_en: Some("unexpected error".to_owned()),
                                msg_en: Some("Something went wrong in database while checking user information.".to_owned()),
                                result_cn: None,
                                msg_cn: None,
                            });
                    },
                    Ok(inner_res) => { 
                        match inner_res {
                            Err(_) => {
                                let register_res = data.db
                                    .send(ResgisterMessage{
                                        username: form.mobile.clone(),
                                        email: None,
                                        mobile: Some(form.mobile.clone()),
                                        password: real_verification_code,
                                        role: customize_role("net_friend"),
                                        job_number: None,
                                    }).await;
                                match register_res {
                                    Err(_) => { 
                                        return HttpResponse::InternalServerError().json(
                                            OperationResult {
                                                result_en: Some("unexpected error".to_owned()),
                                                msg_en: Some("Something went wrong in database while auto creating user.".to_owned()),
                                                result_cn: None,
                                                msg_cn: None,
                                            });
                                    },
                                    Ok(handler_result) => { 
                                        match handler_result {
                                            Err(msg) => { 
                                                return HttpResponse::BadRequest().json(
                                                    OperationResult {
                                                        result_en: Some("error".to_owned()),
                                                        msg_en: Some(format!("Register failed.\n{}.", msg)),
                                                        result_cn: None,
                                                        msg_cn: None,
                                                    });
                                            },
                                            Ok(user) => { 
                                                let mut lock = VERIFICATION_MAP.write().unwrap();
                                                lock.remove(&form.mobile);
                                                id.remember(user.id.to_string());
                                                return HttpResponse::Ok().json(
                                                    OperationResult {
                                                        result_en: Some("success".to_owned()),
                                                        msg_en: Some(format!("Successfully registered.\nWelcome {}.\nYour password is the verification code.", user.username)),
                                                        result_cn: None,
                                                        msg_cn: None,
                                                    });
                                            },
                                        }
                                    }
                                };
                            },
                            Ok(inner_user) => { user = inner_user; },
                        }
                    },    
                }
                let mut lock = VERIFICATION_MAP.write().unwrap();
                lock.remove(&form.mobile);
                id.remember(user.id.to_string());
                HttpResponse::Ok().json(
                    OperationResult {
                        result_en: Some("success".to_owned()),
                        msg_en: Some(format!("Welcome {}.", user.username)),
                        result_cn: None,
                        msg_cn: None,
                    })
            } else {
                let mut lock = VERIFICATION_MAP.write().unwrap();
                lock.remove(&form.mobile);
                HttpResponse::BadRequest().json(
                    OperationResult {
                        result_en: Some("rejected".to_owned()),
                        msg_en: Some("Your verification code is out of date.".to_owned()),
                        result_cn: None,
                        msg_cn: None,
                    })
            }
        } else {
            HttpResponse::BadRequest().json(
                OperationResult {
                    result_en: Some("rejected".to_owned()),
                    msg_en: Some("Your verification code is not correct.".to_owned()),
                    result_cn: None,
                    msg_cn: None,
                })
        }
    } else {
        HttpResponse::BadRequest().json(
            OperationResult {
                result_en: Some("rejected".to_owned()),
                msg_en: Some("Please require verification code first.".to_owned()),
                result_cn: None,
                msg_cn: None,
            })
    }
}

pub async fn logout(
    id: Identity,
) -> HttpResponse {
    if !id.identity().is_none() {
        id.forget();
        HttpResponse::Ok().json(
            OperationResult {
                result_en: Some("success".to_owned()),
                msg_en: Some("Logout successfully.".to_owned()),
                result_cn: None,
                msg_cn: None,
            })
    } else {
        HttpResponse::BadRequest().json(
            OperationResult {
                result_en: Some("rejected".to_owned()),
                msg_en: Some("You are not online now.".to_owned()),
                result_cn: None,
                msg_cn: None,
            })
    }
}