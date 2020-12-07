use crate::{
    database::*,
    user::model::*,
    utils::role_filter::{ customize_role, role_level },
    utils::operation_result::OperationResult,
    errors::ServiceError
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

pub async fn me (
    data: web::Data<DBState>,
    id: Identity,
) -> HttpResponse {
    if let Some(user_id) = id.identity() {
        let get_user_res = data.db
            .send(UserId(user_id.parse::<i32>().unwrap())).await;
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
                        HttpResponse::BadRequest().json(
                            OperationResult {
                                result_en: Some("error".to_owned()),
                                msg_en: Some(format!("Get User information failed.\n{}.", msg)),
                                result_cn: None,
                                msg_cn: None,
                            })
                    },
                    Ok(user) => { HttpResponse::Ok().json(user) },
                }
            },
        }
    } else {
        HttpResponse::Unauthorized().json(
            OperationResult {
                result_en: Some("rejected".to_owned()),
                msg_en: Some("You are not logined now.".to_owned()),
                result_cn: None,
                msg_cn: None,
            }) 
    }
}

pub async fn auth_check(
    data: web::Data<DBState>,
    id: Identity,
    mut allowed_lowest_role: String,
) -> Result<(), ServiceError> {
    allowed_lowest_role = customize_role(&allowed_lowest_role);
    info!("allowed_lowest_role: {}", allowed_lowest_role);
    if let Some(user_id) = id.identity() {
        id.remember(user_id.clone());
        let get_user_res = data.db
            .send(UserId(user_id.parse::<i32>().unwrap())).await;
        match get_user_res {
            Err(_) => { Err(ServiceError::InternalServerError) },
            Ok(inner_res) => { 
                match inner_res {
                    Err(_) => { Err(ServiceError::Unauthorized) },
                    Ok(user) => {
                        if role_level(&allowed_lowest_role) <= role_level(&user.role) { Ok(()) }
                        else { Err(ServiceError::Unauthorized) }
                    },
                }
            },
        }
    } else { Err(ServiceError::NotLogined) }
}