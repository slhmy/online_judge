pub struct QueryRoot;
use futures::executor;
use super::Context;

use uuid::Uuid;

use crate::{
    user::{
        service::{
            catalog::{ GetUserCatalogMessage, get_user_catalog_service, UserCatalog },
        }
    },
    region::{
        model::OutRegion,
        service::{
            tags::{ GetRegionTagsMessage, get_region_tags_service, OutTags },
            info::{ get_region_service, GetRegionMessage },
        }
    },
    problem::{
        model::OutProblem,
        service::{
            catalog::{ GetProblemCatalogForm,ProblemCatalog, get_problem_catalog_service },
            content::{ get_problem as get_problem_service }
        },
    },
    judge_server::service::{
        info::{ OutJudgeServerInfo,server_info as server_info_service },
    },
    status::service::{
        catalog::{ StatusCatalog, get_status_catalog_service },
        get::{ GetStatusMessage, get_status_service, DetailedStatus },
    },
    contest::rank::acm::{ get_acm_rank_service, GetACMRankMessage, ACMRank },
    contest::service::{
        get::{ get_contest_service, GetContestForm },
        catalog::{ ContestCatalog, ContestCatalogElement, get_contest_catalog_service, GetContestCatalogForm },
    },
    test_case::service::{
        catalog::{ TestCaseCatalog, get_test_case_catalog_service }
    },
    errors::ServiceResult,
};

#[juniper::object(Context = Context)]
/// This is the root for all kinds of queries, you can get any thing avaliabe here
impl QueryRoot {

    fn region(context: &Context, name: String) -> ServiceResult<OutRegion> {
        executor::block_on(get_region_service(
            context.db.clone(),
            GetRegionMessage {
                name: name,
            },
            context.id.clone(),
        ))
    }

    fn region_tags(context: &Context, region: String) -> ServiceResult<OutTags> {
        executor::block_on(get_region_tags_service(
            context.db.clone(),
            GetRegionTagsMessage {
                region: region,
            },
            context.id.clone(),
        ))
    }

    fn problem(
        context: &Context, 
        id: i32, 
        region: String
    ) -> ServiceResult<OutProblem> {
        executor::block_on(get_problem_service(context.db.clone(), id, region, context.id.clone()))
    }

    fn acm_rank(
        context: &Context, 
        region: String,
        columes_per_page: Option<i32>,
    ) -> ServiceResult<ACMRank> {
        executor::block_on(get_acm_rank_service(
            context.db.clone(),
            GetACMRankMessage {
                region: region,
                columes_per_page: columes_per_page,
            },
            context.id.clone(),
        ))
    }

    fn status(
        context: &Context, 
        id: Uuid, 
    ) -> ServiceResult<DetailedStatus> {
        executor::block_on(get_status_service(context.db.clone(), GetStatusMessage{ id:id }, context.id.clone()))
    }

    fn user_catalog(
        context: &Context,
        id: Option<i32>,
        username: Option<String>,
        mobile: Option<String>,
        email: Option<String>,
        job_number: Option<String>,
        elements_per_page: Option<i32>,
    ) -> ServiceResult<UserCatalog> {
        executor::block_on(get_user_catalog_service(
            context.db.clone(), 
            GetUserCatalogMessage {
                id: id,
                username: username,
                mobile: mobile,
                email: email,
                job_number: job_number,
                elements_per_page: elements_per_page
            },
            context.id.clone())
        )
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
        elements_per_page: Option<i32>,
        title: Option<String>,
        tags: Option<Vec<String>>,
        difficulty: Option<String>,
    ) -> ServiceResult<ProblemCatalog> {
        executor::block_on(get_problem_catalog_service(
            context.db.clone(),
            GetProblemCatalogForm {
                region: region,
                elements_per_page: elements_per_page,
                title: title,
                tags: tags,
                difficulty: difficulty,
            },
            context.id.clone(),
        ))
    }

    fn contest(
        context: &Context,
        region: String,
    ) -> ServiceResult<ContestCatalogElement> {
        executor::block_on(get_contest_service(
            context.db.clone(),
            GetContestForm{ 
                region: region, 
            },
            context.id.clone()))
    }

    fn contest_catalog(
        context: &Context,
        name: Option<String>,
        elements_per_page: Option<i32>,
    ) -> ServiceResult<ContestCatalog> {
        executor::block_on(get_contest_catalog_service(
            context.db.clone(),
            GetContestCatalogForm{ 
                name: name, 
                elements_per_page: elements_per_page
            },
            context.id.clone()))
    }

    fn test_case_catalog(
        context: &Context,
        elements_per_page: Option<i32>,
    ) -> ServiceResult<TestCaseCatalog> {
        executor::block_on(get_test_case_catalog_service(context.db.clone(), elements_per_page, context.id.clone()))
    }

    fn judge_servers(
        context: &Context,
    ) -> ServiceResult<Vec<OutJudgeServerInfo>> {
        executor::block_on(server_info_service(context.id.clone()))
    }
}