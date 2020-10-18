use crate::{
    database::*,
    user::model::*,
    encryption::encode::make_hash,
    utils::{
        role_filter::customize_role,
        regex_matcher::RegexMatcher,
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
    pub hash: Option<Vec<u8>>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub role: Option<String>,
    pub job_number: Option<String>,
}

fn get_user_change(req: UserChangeRequest, user: User) -> UserChange {
    let hash = if req.password.is_none() { None }
    else {
        if req.password.clone().unwrap().is_password() { Some(make_hash(&req.password.unwrap(), &user.salt).to_vec()) }
        else { None }
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
        hash: hash,
        email: email,
        mobile: mobile,
        role: role,
        job_number: job_number,
    }
}

impl Message for UserChangeRequest {
    type Result = Result<OutUser, String>;
}

impl Handler<UserChangeRequest> for DbExecutor {
    type Result = Result<OutUser, String>;

    fn handle(&mut self, msg: UserChangeRequest, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let operation_result = users.filter(id.eq(msg.id)).limit(1).load::<User>(&self.0).expect("Error loading user").pop();

        if !operation_result.is_none() {
            let user = operation_result.unwrap();
            let msg_id = msg.id;
            let user_change = get_user_change(msg, user);
            let inner_result = diesel::update(users.filter(id.eq(msg_id)))
                .set(user_change)
                .get_result::<User>(&self.0);
            match inner_result {
                Err(system_msg) => Err(format!("Database operate failed.\nSystem_msg: {}", system_msg)),
                Ok(user) => Ok(OutUser::from(user)),
            }
        } else {
            Err("Database operate failed.\nReason: can't find account.".to_owned())
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

        let operation_result = users.filter(id.eq(msg.id)).limit(1).load::<User>(&self.0).expect("Error loading user").pop();

        if !operation_result.is_none() {
            let user = operation_result.unwrap();
            let inner_result = diesel::delete(users.filter(id.eq(msg.id)))
                .execute(&self.0);
            match inner_result {
                Err(system_msg) => Err(format!("Database operate failed.\nSystem_msg: {}", system_msg)),
                Ok(_) => Ok(OutUser::from(user)),
            }
        } else {
            Err("Database operate failed.\nReason: can't find account.".to_owned())
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserChangeRequest {
    pub id: i32,
    pub password: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub role: Option<String>,
    pub job_number: Option<String>,
}

pub async fn change_info(
    data: web::Data<State>,
    form: web::Form<UserChangeRequest>,
    id: Identity,
) -> HttpResponse {
    let res;
    if let Some(user_id) = id.identity() {
        let get_user_res = data.db
            .send(UserId(user_id.parse::<i32>().unwrap())).await;
        let user;
        match get_user_res {
            Err(_) => {return HttpResponse::InternalServerError().body("Unexpected Database error."); },
            Ok(inner_res) => { 
                match inner_res {
                    Err(msg) => { return HttpResponse::BadRequest().body(format!("Get User information failed.\n{}.", msg)); },
                    Ok(inner_user) => { user = inner_user; },
                }
            },
        }

        if user.role == "admin".to_owned() || user.id == form.id {
            res = data.db.send(form.into_inner()).await;
        } else {
            return HttpResponse::BadRequest().body("You are not allowed to change other user's information.");
        }
    } else {
        return HttpResponse::BadRequest().body("You are not logined now.");   
    }
    
    match res {
        Err(msg) => { HttpResponse::BadRequest().body(format!("Change information failed.\n{}.", msg)) }
        Ok(_res) => { HttpResponse::Ok().body("Change information successfully.") }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserDeleteRequest{
    id: i32,
}

pub async fn delete_user(
    data: web::Data<State>,
    form: web::Form<UserDeleteRequest>,
    id: Identity,
) -> HttpResponse {
    let res;
    if let Some(user_id) = id.identity() {
        let get_user_res = data.db
            .send(UserId(user_id.parse::<i32>().unwrap())).await;
        let user;
        match get_user_res {
            Err(_) => {return HttpResponse::InternalServerError().body("Unexpected Database error."); },
            Ok(inner_res) => { 
                match inner_res {
                    Err(msg) => { return HttpResponse::BadRequest().body(format!("Get User information failed.\n{}.", msg)); },
                    Ok(inner_user) => { user = inner_user; },
                }
            },
        }

        if user.role == "admin".to_owned() {
            res = data.db
                .send(form.into_inner()).await;
        } else {
            return HttpResponse::BadRequest().body("You are not allowed to delete user.");
        }
    } else {
        return HttpResponse::BadRequest().body("You are not logined now.");   
    }
    
    match res {
        Err(msg) => { HttpResponse::BadRequest().body(format!("Delete user failed.\n{}.", msg)) }
        Ok(_res) => { HttpResponse::Ok().body("Delete user successfully.") }
    }
}

pub async fn get_all_users(
    data: web::Data<State>,
    id: Identity,
) -> HttpResponse {
    let res;
    if let Some(user_id) = id.identity() {
        let get_user_res = data.db.send(UserId(user_id.parse::<i32>().unwrap())).await;
        let user;
        match get_user_res {
            Err(_) => {return HttpResponse::InternalServerError().body("Unexpected Database error."); },
            Ok(inner_res) => { 
                match inner_res {
                    Err(msg) => { return HttpResponse::BadRequest().body(format!("Get User information failed.\n{}.", msg)); },
                    Ok(inner_user) => { user = inner_user; },
                }
            },
        }

        if user.role == "admin".to_owned() {
            res = data.db.send(AllUsers()).await;
        } else {
            return HttpResponse::BadRequest().body("You are not allowed to get all users.");
        }
    } else {
        return HttpResponse::BadRequest().body("You are not logined now.");   
    }
    
    match res {
        Err(msg) => { HttpResponse::BadRequest().body(format!("Get all users failed.\n{}.", msg)) }
        Ok(res) => { HttpResponse::Ok().json(res) }
    }
}