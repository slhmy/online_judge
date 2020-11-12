use crate::{
    database::*,
    errors::{ServiceError, ServiceResult},
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use uuid::Uuid;
use actix_identity::Identity;

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct OwnerPreview {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct ProblemPreview {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct StatusCatalogElement {
    pub id: Uuid,
    pub owner: OwnerPreview,
    pub problem: ProblemPreview,
    pub language: String,
    pub state: String,
    pub judge_type: String,
    pub result: Option<String>,
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct StatusCatalog {
    pub total_count: i32,
    pub elements: Vec<StatusCatalogElement>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetStatusCatalogMessage {
    pub region: String,
    pub count_per_page: i32,
    pub problem_id: Option<i32>,
    pub problem_title: Option<String>,
    pub user_id: Option<i32>,
    pub username: Option<String>,
    pub language: Option<String>,
    pub page_number: i32,
}

impl Message for GetStatusCatalogMessage {
    type Result = Result<StatusCatalog, String>;
}

impl Handler<GetStatusCatalogMessage> for DbExecutor {
    type Result = Result<StatusCatalog, String>;
    
    fn handle(&mut self, msg: GetStatusCatalogMessage, _: &mut Self::Context) -> Self::Result {
        use diesel::dsl::*;
        use crate::schema::{
            problems,
            users,
            status,
        };

        let count_star = status::table
            .filter(status::problem_region.eq(msg.region.clone()))
            .filter(status::problem_id.nullable().eq(msg.problem_id).or(msg.problem_id.is_none()))
            .filter(status::owner_id.nullable().eq(msg.user_id).or(msg.user_id.is_none()))
            .filter(status::language.ilike(
                "%".to_owned() + &msg.language.clone().unwrap_or("".to_owned()) + "%"
            ).or(msg.language.is_none()))
            .inner_join(problems::table.on(status::problem_id.eq(problems::id)
                .and(problems::title.ilike(
                    "%".to_owned() + &msg.problem_title.clone().unwrap_or("".to_owned()) + "%"
                ).or(msg.problem_title.is_none()))))
            .inner_join(users::table.on(status::owner_id.eq(users::id)
                .and(users::username.ilike(
                    "%".to_owned() + &msg.username.clone().unwrap_or("".to_owned()) + "%"
                ).or(msg.username.is_none()))))
            .select(count_star())
            .first::<i64>(&self.0)
            .expect("Error counting status.");

        let status_vec = status::table
            .filter(status::problem_region.eq(msg.region.clone()))
            .filter(status::problem_id.nullable().eq(msg.problem_id).or(msg.problem_id.is_none()))
            .filter(status::owner_id.nullable().eq(msg.user_id).or(msg.user_id.is_none()))
            .filter(status::language.ilike(
                "%".to_owned() + &msg.language.clone().unwrap_or("".to_owned()) + "%"
            ).or(msg.language.is_none()))
            .inner_join(problems::table.on(status::problem_id.eq(problems::id)
                .and(problems::title.ilike(
                    "%".to_owned() + &msg.problem_title.clone().unwrap_or("".to_owned()) + "%"
                ).or(msg.problem_title.is_none()))))
            .inner_join(users::table.on(status::owner_id.eq(users::id)
                .and(users::username.ilike(
                    "%".to_owned() + &msg.username.clone().unwrap_or("".to_owned()) + "%"
                ).or(msg.username.is_none()))))
            .select((
                status::id,
                status::problem_id,
                problems::title,
                status::owner_id,
                users::username,
                status::language,
                status::state,
                status::judge_type,
                status::result,
                status::score,
            ))
            .offset(((msg.page_number - 1) * msg.count_per_page) as i64)
            .limit(msg.count_per_page as i64)
            .load::<(
                Uuid,
                i32,
                String,
                i32,
                String,
                String,
                String,
                String,
                Option<String>,
                Option<f64>,
            )>(&self.0)
            .expect("Error loading status.");

        let mut catalog = StatusCatalog {
            total_count: count_star as i32,
            elements: Vec::new(),
        };

        for (
            t_id,
            t_problem_id,
            t_problem_title,
            t_owner_id,
            t_username,
            t_language,
            t_state,
            t_judge_type,
            t_result,
            t_score,
        ) in status_vec {
            catalog.elements.push(StatusCatalogElement{
                id: t_id,
                owner: OwnerPreview {
                    id: t_owner_id,
                    username: t_username,
                },
                problem: ProblemPreview {
                    id: t_problem_id,
                    title: t_problem_title,
                },
                language: t_language,
                state: t_state,
                judge_type: t_judge_type,
                result: t_result,
                score: t_score,
            });
        }

        Ok(catalog)
    }
}

pub async fn get_status_catalog_service(
    data: web::Data<DBState>,
    region: String,
    count_per_page: i32,
    problem_id: Option<i32>,
    problem_title: Option<String>,
    user_id: Option<i32>,
    username: Option<String>,
    language: Option<String>,
    page_number: i32,
    _id: Identity,
) -> ServiceResult<StatusCatalog> {
    if count_per_page <= 0 { return Err(ServiceError::BadRequest("Count per page should be larger than 0.".to_owned())); }
    if page_number <= 0 { return Err(ServiceError::BadRequest("Page number should be larger than 0.".to_owned())); }

    let db_result = data.db.send(GetStatusCatalogMessage {
        region: region,
        count_per_page: count_per_page,
        problem_id: problem_id,
        problem_title: problem_title,
        user_id: user_id,
        username: username,
        page_number: page_number,
        language: language,
    }).await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(catalog) => Ok(catalog),
            }
        }
    }
}