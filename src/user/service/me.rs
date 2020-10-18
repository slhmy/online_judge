use crate::{
    database::*,
    user::model::*,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

pub async fn me (
    data: web::Data<State>,
    id: Identity,
) -> HttpResponse {
    if let Some(user_id) = id.identity() {
        let get_user_res = data.db
            .send(UserId(user_id.parse::<i32>().unwrap())).await;
        match get_user_res {
            Err(_) => {return HttpResponse::InternalServerError().body("Unexpected Database error."); },
            Ok(inner_res) => { 
                match inner_res {
                    Err(msg) => { HttpResponse::BadRequest().body(format!("Get User information failed.\n{}.", msg)) },
                    Ok(user) => { HttpResponse::Ok().json(user) },
                }
            },
        }
    } else {
        HttpResponse::BadRequest().body("You are not logined now.")  
    }
}