pub struct QueryRoot;
use futures::executor;
use juniper::FieldResult;
use super::Context;
use super::objects::{
    starwar::*,
    user::*,
    tag::*,
};

use crate::service::{
    user_service::*,
    problem_service::*,
};

use crate::{
    problem::service::{
        catalog::{
            ProblemCatalog,
            get_problem_catalog_service,
        },
        content::{
            OutProblem, 
            get_problem as get_problem_service,
        }
    },
    judge_server::service::{
        info::{
            OutJudgeServerInfo,
            server_info as server_info_service,
        },
    },
    status::service::{
        catalog::{
            StatusCatalog,
            get_status_catalog_service,
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

    fn status_catalog(
        context: &Context,
        region: Option<String>,
        count_per_page: i32,
        problem_id: Option<i32>,
        problem_title: Option<String>,
        user_id: Option<i32>,
        username: Option<String>,
        language: Option<String>,
        page_number: i32,
    ) -> ServiceResult<StatusCatalog> {
        executor::block_on(get_status_catalog_service(
            context.db.clone(),
            region,
            count_per_page,
            problem_id,
            problem_title,
            user_id,
            username,
            language,
            page_number,
            context.id.clone())
        )
    }

    fn problem_catalog(
        context: &Context,
        region: String, 
        problems_per_page: Option<i32>,
    ) -> ServiceResult<ProblemCatalog> {
        executor::block_on(get_problem_catalog_service(context.db.clone(), region.clone(), problems_per_page))
    }

    fn judge_servers(
        context: &Context,
    ) -> ServiceResult<Vec<OutJudgeServerInfo>> {
        executor::block_on(server_info_service(context.id.clone()))
    }
}