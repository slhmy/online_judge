use crate::{
    test_case::utils::make,
    database::*,
    test_case::model::TestCase,
    errors::{ ServiceError, ServiceResult },
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;
use super::get::GetTestCaseMessage;

impl Message for UpdateTestCaseMessage {
    type Result = Result<TestCase, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTestCaseMessage {
    pub name: String,
    pub is_spj: bool,
    pub count: i32,
}

impl Handler<UpdateTestCaseMessage> for DbExecutor {
    type Result = Result<TestCase, String>;

    fn handle(&mut self, msg: UpdateTestCaseMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::test_cases::dsl::*;

        let result = diesel::update(test_cases)
            .filter(name.eq(msg.name))
            .set((is_spj.eq(msg.is_spj),count.eq(msg.count)))
            .get_result::<TestCase>(&self.0);

        match result {
            Err(_) => { Err("Error while creating update test_case.".to_owned()) },
            Ok(inner_result) => { Ok(TestCase::from(inner_result)) }
        }
    }
}

pub async fn update_test_case_service(
    data: web::Data<DBState>,
    bytes: &[u8],
    name: String,
    is_spj: bool,
    _id: Identity,
) -> ServiceResult<TestCase> {
    let db_result = data.db.send(GetTestCaseMessage {
        name: name.clone(),
    }).await;

    match db_result {
        Err(_) => return Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(_) => return Err(ServiceError::BadRequest("Cannot find test_case.".to_owned())),
                Ok(_) => {},
            }
        }
    }

    let count = if is_spj { 
        make::make_spj_info(name.clone(), &bytes)
    } else {
        make::make_normal_info(name.clone(), &bytes)
    };
    if count == 0 {
        return Err(ServiceError::BadRequest("Less than one effective test case is found.".to_owned()));
    }

    let db_result = data.db.send(UpdateTestCaseMessage {
        name: name,
        is_spj: is_spj,
        count: count,
    }).await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(test_case) => Ok(test_case),
            }
        }
    }
}