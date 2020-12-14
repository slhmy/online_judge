use crate::{
    database::*,
    user::model::*,
    user::service::me::auth_check,
    utils::{
        encryption::encode::make_hash,
        role_filter::customize_role,
        regex_matcher::RegexMatcher,
        operation_result::OperationResult,
    },
};
use actix::prelude::*;
use actix_web::{HttpResponse, web};
use actix_identity::Identity;
use diesel::prelude::*;
use crate::schema::users;

#[derive(Debug, Clone, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct UserChange {
    pub username: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub role: Option<String>,
    pub job_number: Option<String>,
    pub hash: Vec<u8>,
}

fn get_user_change(req: UserChangeRequest, user: User) -> UserChange {
    let username = if req.username.is_none() { None }
    else { Some(req.username.unwrap()) };

    let hash = if req.password.is_none() { user.hash }
    else {
        if req.password.clone().unwrap().is_password() { make_hash(&req.password.unwrap(), &user.salt).to_vec() }
        else { user.hash }
    };

    let email = if req.email.is_none() { None }
    else {
        if req.email.clone().unwrap().is_email() { Some(req.email.unwrap()) }
        else { None }
    };

    let mobile = if req.mobile.is_none() { None }
    else {
        if req.mobile.clone().unwrap().is_mobile() { Some(req.mobile.unwrap()) }
        else { None }
    };

    let role = if req.role.is_none() { None }
    else { Some(customize_role(&req.role.unwrap())) };

    let job_number = if req.job_number.is_none() { None }
    else { Some(req.job_number.unwrap()) };

    UserChange {
        username: username,
        email: email,
        mobile: mobile,
        role: role,
        job_number: job_number,
        hash: hash,
    }
}

impl Message for UserChangeRequest {
    type Result = Result<OutUser, String>;
}

impl Handler<UserChangeRequest> for DbExecutor {
    type Result = Result<OutUser, String>;

    fn handle(&mut self, msg: UserChangeRequest, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let operation_result = users.filter(id.eq(msg.id)).first::<User>(&self.0);
        
        match operation_result {
            Ok(result) => {
                let user = result;
                let msg_id = msg.id;
                let user_change = get_user_change(msg, user);
                let affected_rows = diesel::update(users.filter(id.eq(msg_id)))
                    .set(user_change)
                    .execute(&self.0).unwrap_or(0);
                let inner_result;
                if affected_rows == 1 {
                    inner_result = users.find(msg_id).first::<User>(&self.0);
                } else {
                    return Err(format!("Database operate failed.\nReason: conflict identity_info"));
                }
                match inner_result {
                    Err(system_msg) => Err(format!("Database operate failed.\nSystem_msg: {}", system_msg)),
                    Ok(user) => Ok(OutUser::from(user)),
                }
            },
            Err(msg) => {
                Err(format!("Database operate failed.\nSystem_msg: {}", msg))
            },
        }
    }
}

impl Message for UserDeleteRequest {
    type Result = Result<OutUser, String>;
}

impl Handler<UserDeleteRequest> for DbExecutor {
    type Result = Result<OutUser, String>;

    fn handle(&mut self, msg: UserDeleteRequest, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let operation_result = users.filter(id.eq(msg.id)).first::<User>(&self.0);

        match operation_result {
            Ok(result) => {
                let user = result;
                let inner_result = diesel::delete(users.filter(id.eq(msg.id)))
                    .execute(&self.0);
                match inner_result {
                    Err(system_msg) => Err(format!("Database operate failed.\nSystem_msg: {}", system_msg)),
                    Ok(_) => Ok(OutUser::from(user)),
                }
            },
            Err(msg) => {
                Err(format!("Database operate failed.\nSystem_msg: {}", msg))
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserChangeRequest {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub role: Option<String>,
    pub job_number: Option<String>,
}

pub async fn change_info(
    data: web::Data<DBState>,
    form: web::Form<UserChangeRequest>,
    id: Identity,
) -> HttpResponse {
    let res;
    if let Some(user_id) = id.identity() {
        let get_user_res = data.db
            .send(UserId(user_id.parse::<i32>().unwrap())).await;
        let user;
        match get_user_res {
            Err(_) => { 
                return HttpResponse::InternalServerError().json(
                    OperationResult {
                        result_en: Some("unexpected error".to_owned()),
                        msg_en: Some("Something went wrong in database".to_owned()),
                        result_cn: None,
                        msg_cn: None,
                    });
            },
            Ok(inner_res) => { 
                match inner_res {
                    Err(msg) => { 
                        return HttpResponse::BadRequest().json(
                            OperationResult {
                                result_en: Some("error".to_owned()),
                                msg_en: Some(format!("Get User information failed.\n{}.", msg)),
                                result_cn: None,
                                msg_cn: None,
                            })
                    },
                    Ok(inner_user) => { user = inner_user; },
                }
            },
        }

        if user.role == "admin".to_owned() || (user.id == form.id && form.role.is_none()) {
            res = data.db.send(form.into_inner()).await;
        } else {
            return HttpResponse::BadRequest().json(
                OperationResult {
                    result_en: Some("rejected".to_owned()),
                    msg_en: Some("You are not allowed to change other user's information or change your role.".to_owned()),
                    result_cn: None,
                    msg_cn: None,
                });
        }
    } else {
        return HttpResponse::BadRequest().json(
            OperationResult {
                result_en: Some("rejected".to_owned()),
                msg_en: Some("You are not logined now.".to_owned()),
                result_cn: None,
                msg_cn: None,
            });
    }

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
                        result_en: Some("error".to_owned()),
                        msg_en: Some(format!("Change information failed.\n{}.", msg)),
                        result_cn: None,
                        msg_cn: None,
                    }),
                Ok(_) => HttpResponse::Ok().json(
                    OperationResult {
                        result_en: Some("success".to_owned()),
                        msg_en: Some("Change information successfully.".to_owned()),
                        result_cn: None,
                        msg_cn: None,
                    }),
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserDeleteRequest{
    id: i32,
}

pub async fn delete_user(
    data: web::Data<DBState>,
    form: web::Form<UserDeleteRequest>,
    id: Identity,
) -> HttpResponse {
    let res;

    match auth_check(data.clone(), id.clone(), "admin".to_owned()).await {
        Ok(_) => {
            res = data.db.send(form.into_inner()).await;
        },
        Err(_) => {
            return HttpResponse::BadRequest().json(
                OperationResult {
                    result_en: Some("rejected".to_owned()),
                    msg_en: Some("You are not allowed to delete user.".to_owned()),
                    result_cn: None,
                    msg_cn: None,
                });
        }
    }
    
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
                        result_en: Some("error".to_owned()),
                        msg_en: Some(format!("Delete user failed.\n{}.", msg)),
                        result_cn: None,
                        msg_cn: None,
                    }),
                Ok(_res) => HttpResponse::Ok().json(
                    OperationResult {
                        result_en: Some("success".to_owned()),
                        msg_en: Some("Delete user successfully.".to_owned()),
                        result_cn: None,
                        msg_cn: None,
                    }),
            }
        }
    }
}

pub async fn get_all_users(
    data: web::Data<DBState>,
    id: Identity,
) -> HttpResponse {
    let res;
    if let Some(user_id) = id.identity() {
        let get_user_res = data.db.send(UserId(user_id.parse::<i32>().unwrap())).await;
        let user;
        match get_user_res {
            Err(_) => {
                return HttpResponse::InternalServerError().json(
                    OperationResult {
                        result_en: Some("unexpected error".to_owned()),
                        msg_en: Some("Unexpected Database error.".to_owned()),
                        result_cn: None,
                        msg_cn: None,
                    });
            },
            Ok(inner_res) => { 
                match inner_res {
                    Err(msg) => { 
                        return HttpResponse::BadRequest().json(
                            OperationResult {
                                result_en: Some("error".to_owned()),
                                msg_en: Some(format!("Get User information failed.\n{}.", msg)),
                                result_cn: None,
                                msg_cn: None,
                            });
                    },
                    Ok(inner_user) => { user = inner_user; },
                }
            },
        }

        if user.role == "admin".to_owned() {
            res = data.db.send(AllUsers()).await;
        } else {
            return HttpResponse::BadRequest().json(
                OperationResult {
                    result_en: Some("rejected".to_owned()),
                    msg_en: Some("You are not allowed to get all users.".to_owned()),
                    result_cn: None,
                    msg_cn: None,
                });
        }
    } else {
        return HttpResponse::BadRequest().json(
            OperationResult {
                result_en: Some("rejected".to_owned()),
                msg_en: Some("You are not logined now.".to_owned()),
                result_cn: None,
                msg_cn: None,
            });
    }
    
    match res {
        Err(_) => HttpResponse::InternalServerError().json(
            OperationResult {
                result_en: Some("unexpected error".to_owned()),
                msg_en: Some("Unexpected Database error.".to_owned()),
                result_cn: None,
                msg_cn: None,
            }),
        Ok(handler_result) => { 
            match handler_result {
                Err(msg) => HttpResponse::BadRequest().json(
                    OperationResult {
                        result_en: Some("error".to_owned()),
                        msg_en: Some(format!("Get all users failed.\n{}.", msg)),
                        result_cn: None,
                        msg_cn: None,
                    }),
                Ok(all_users) => HttpResponse::Ok().json(all_users),
            }
        }
    }
}