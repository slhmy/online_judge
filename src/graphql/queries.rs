pub struct QueryRoot;
use futures::executor;
use juniper::FieldResult;
use super::Context;
use super::objects::{
    starwar::*,
    user::*,
    status::*,
    tag::*,
};

use crate::service::{
    user_service::*,
    problem_service::*,
};

use crate::{
    problem::service::{
        catalog::{
            Catalog,
            get_catalog as get_catalog_service,
        },
        content::{
            OutProblem, 
            get_problem as get_problem_service,
        }
    },
    errors::ServiceResult,
};

#[juniper::object(Context = Context)]
/// This is the root for all kinds of queries, you can get any thing avaliabe here
impl QueryRoot {
    /// Starwar query example
    fn human(id: String) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }

    /// Use query by token to get all the data related with user.
    fn user(token: String) -> FieldResult<User> {
        Ok(get_user_by_token(&token))
    }

    fn get_all_tags() -> FieldResult<Vec<Tag>> {
        Ok(
            vec![Tag {
                id: 1,
                name: "Basic".to_owned(),
                related_problems: vec![get_problem_by_id(1)],
            }]
        )
    }

    fn problem(
        context: &Context, 
        id: i32, 
        region: String
    ) -> ServiceResult<OutProblem> {
        executor::block_on(get_problem_service(context.db.clone(), id, region, context.id.clone()))
    }

    fn status(id: String) -> FieldResult<Status> {
        Ok(Status {
            id: 1,
            information: StatusInformation {
                problem: get_problem_by_id(1),
                owner: get_user_by_token(""),
                region: "Global".to_owned(),
                submit_time: "2020/10/5 20:45:00".to_owned(),
                finish_time: "2020/10/5 20:45:02".to_owned(),
                judge_strategy: JudgeStrategy::ACM,
                final_result: "Accepted".to_owned(),
            },
            is_compile_error: false,
            compile_error_message: None,
            judge_details: Some(
                vec![JudgeResult{
                    test_case: 1,
                    result: ResultType::Success,
                    cpu_time: 1,
                    real_time: 2,
                    memory: 1671168,
                    signal: 0,
                    exit_code: 0,
                    error: 0,
                    output: "3\n".to_owned(),
                }]
            ),
            lowest_user_identity: UserIdentity::Teacher,
            special_permissions_key: Some("YOUR_PERMISSIONS_KEY".to_owned()),
        })
    }

    async fn catalog(
        context: &Context,
        region: String, 
        problems_per_page: Option<i32>,
    ) -> ServiceResult<Catalog> {
        executor::block_on(get_catalog_service(context.db.clone(), region.clone(), problems_per_page))
    }
}