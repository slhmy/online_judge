use crate::{
    schema::contest_register_lists,
    database::*,
    contest::model::RegisterInfo,
    errors::{ ServiceError, ServiceResult },
    region::model::Region,
    utils::encryption::encode::make_hash,
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;
use atoi::atoi;

#[derive(Debug, Clone, Deserialize, Insertable, Queryable)]
#[table_name = "contest_register_lists"]
struct InsertableRegisterInfo {
    contest_region: String,
    user_id: i32,
    is_unrated: bool,
}

impl Message for NewRegisterInfoMessage {
    type Result = Result<RegisterInfo, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewRegisterInfoMessage {
    pub contest_region: String,
    pub user_id: i32,
    pub is_unrated: bool,
    pub password: Option<String>,
}

impl Handler<NewRegisterInfoMessage> for DbExecutor {
    type Result = Result<RegisterInfo, String>;

    fn handle(&mut self, msg: NewRegisterInfoMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::contest_register_lists::dsl::contest_register_lists;
        use crate::schema::regions::dsl::*;

        let result = regions
            .filter(name.eq(msg.contest_region.clone()))
            .first::<Region>(&self.0);

        match result {
            Err(_) => { return Err("Error while getting region information.".to_owned()); },
            Ok(region) => {
                let premission = if region.need_pass {
                    if msg.password.is_none() { false } 
                    else {
                        if make_hash(&(msg.password.unwrap()), &(region.salt.unwrap())) == region.hash.unwrap().as_ref() {
                            true
                        } else { false }
                    }
                } else { true };

                if premission {
                    let result = diesel::insert_into(contest_register_lists)
                        .values(&InsertableRegisterInfo{
                            contest_region: msg.contest_region,
                            user_id: msg.user_id,
                            is_unrated: msg.is_unrated,
                        })
                        .get_result::<RegisterInfo>(&self.0);

                    match result {
                        Err(_) => { Err("Error while registering.".to_owned()) },
                        Ok(inner_result) => { Ok(RegisterInfo::from(inner_result)) }
                    }
                } else {
                    Err("Wrong password/Password is not given.".to_owned())
                }
            }
        }
    }
}

pub async fn register_service(
    data: web::Data<DBState>,
    contest_region: String,
    is_unrated: bool,
    password: Option<String>,
    id: Identity,
) -> ServiceResult<RegisterInfo> {

    if id.identity().is_none() {
        return Err(ServiceError::Unauthorized);
    }

    let cur_id = id.identity().unwrap();
    id.remember(cur_id.clone());

    let db_result = data.db.send(NewRegisterInfoMessage {
        contest_region: contest_region,
        user_id: atoi::<i32>(cur_id.as_bytes()).unwrap(),
        is_unrated: is_unrated,
        password: password,
    }).await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(region) => Ok(region),
            }
        }
    }
}