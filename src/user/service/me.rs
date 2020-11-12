use crate::{
    database::*,
    user::model::*,
    utils::operation_result::OperationResult,
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
        HttpResponse::BadRequest().json(
            OperationResult {
                result_en: Some("rejected".to_owned()),
                msg_en: Some("You are not logined now.".to_owned()),
                result_cn: None,
                msg_cn: None,
            }) 
    }
}