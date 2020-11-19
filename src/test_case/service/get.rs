use crate::{
    database::*,
    test_case::model::TestCase,
};
use diesel::prelude::*;
use actix::prelude::*;

impl Message for GetTestCaseMessage {
    type Result = Result<TestCase, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTestCaseMessage {
    pub name: String,
}

impl Handler<GetTestCaseMessage> for DbExecutor {
    type Result = Result<TestCase, String>;

    fn handle(&mut self, msg: GetTestCaseMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::test_cases::dsl::*;

        let result = test_cases.filter(name.eq(&msg.name))
            .first::<TestCase>(&self.0);

        match result {
            Err(_) => { Err("Error while creating new test_case.".to_owned()) },
            Ok(inner_result) => { Ok(TestCase::from(inner_result)) }
        }
    }
}