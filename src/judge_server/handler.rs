use crate::{
    database::*,
    errors::ServiceError,
};
use actix_web::{HttpResponse, web};
use actix_identity::Identity;

use super::service::judge::{ judge as do_judge, JudgeRequestForm };

pub async fn judge(
    data: web::Data<State>,
    form: web::Form<JudgeRequestForm>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    do_judge(data, form.problem_id, form.region.clone(), form.src.clone(), form.language.clone(), form.judge_type.clone(), id).await
        .map(|res| HttpResponse::Ok().json(&res))
}