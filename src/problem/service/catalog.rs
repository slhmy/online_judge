use crate::{
    database::*,
    errors::{ServiceError, ServiceResult},
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;
use atoi::atoi;

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct ProblemCatalogElement {
    pub id: i32,
    pub title: String,
    pub tags: Vec<String>,
    pub difficulty: String,
    pub accept_times: i32,
    pub submit_times: i32,
    pub accept_rate: f64,
    pub is_passed: bool,
    pub is_tried: bool,
    pub highest_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct ProblemCatalog {
    pub total_count: i32,
    pub elements: Vec<Vec<ProblemCatalogElement>>,
    pub page_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetProblemCatalogForm {
    pub region: String,
    pub elements_per_page: Option<i32>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub difficulty: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetProblemCatalogMessage {
    pub user_id: Option<i32>,
    pub region: String,
    pub elements_per_page: Option<i32>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub difficulty: Option<String>,
}

impl Message for GetProblemCatalogMessage {
    type Result = Result<ProblemCatalog, String>;
}

impl Handler<GetProblemCatalogMessage> for DbExecutor {
    type Result = Result<ProblemCatalog, String>;
    
    fn handle(&mut self, msg: GetProblemCatalogMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::problems::dsl::*;
        use crate::schema::status;

        let search_title = if msg.title.is_some() {
            let result = str::replace(&msg.title.unwrap(), " ", "%");
            Some("%".to_owned() + &result + "%")
        } else { None };

        let search_tags: Vec<String> = if msg.tags.is_some() {
            if msg.tags.clone().unwrap().len() > 0 { msg.tags.unwrap() }
            else { Vec::<String>::new() }
        } else { Vec::<String>::new() };

        let result = problems.filter(region.eq(msg.region.clone()))
            .filter(tags.overlaps_with(search_tags.clone()).or(search_tags.is_empty()))
            .filter(title.ilike(search_title.clone().unwrap_or("".to_owned())).or(search_title.is_none()))
            .filter(difficulty.nullable().eq(msg.difficulty.clone()).or(msg.difficulty.is_none()))
            .order_by(id.asc())
            .select( (id, title, tags, difficulty, accept_times, submit_times) )
            .load::<(i32, String, Vec<String>, String, i32, i32)>(&self.0)
            .expect("Error loading problems.");

        let mut catalog = ProblemCatalog {
            total_count: 0,
            elements: Vec::new(),
            page_count: 0,
        };
        let mut current_page_number = 0;
        let mut page_problem_count = 0;
        catalog.elements.push(Vec::new());
        info!("{:?}", result);
        for (p_id, p_title, p_tags, p_difficulty, p_accept_times, p_submit_times) in result {
            if !msg.elements_per_page.is_none() && msg.elements_per_page.unwrap() == page_problem_count {
                catalog.elements.push(Vec::new());
                current_page_number += 1;
                page_problem_count = 0;
            }
            catalog.elements[current_page_number as usize].push(
                ProblemCatalogElement {
                    id: p_id,
                    title: p_title,
                    tags: p_tags,
                    difficulty: p_difficulty,
                    accept_times: p_accept_times,
                    submit_times: p_submit_times,
                    accept_rate: if p_submit_times == 0 { 0.0 } 
                        else { p_accept_times as f64 / p_submit_times as f64 },
                    is_passed: if msg.user_id.is_none() { false } else {
                        let result: i64 = status::table
                            .filter(status::result.is_not_null())
                            .filter(status::problem_region.eq(msg.region.clone()))
                            .filter(status::problem_id.eq(p_id))
                            .filter(status::result.nullable().eq("Accepted".to_owned()))
                            .filter(status::owner_id.eq(msg.user_id.unwrap()))
                            .count()
                            .get_result(&self.0)
                            .expect("Error loading user's status.");

                        if result > 0 { true } else { false }
                    },
                    is_tried: if msg.user_id.is_none() { false } else {
                        let result: i64 = status::table
                            .filter(status::result.is_not_null())
                            .filter(status::problem_region.eq(msg.region.clone()))
                            .filter(status::problem_id.eq(p_id))
                            .filter(status::owner_id.eq(msg.user_id.unwrap()))
                            .count()
                            .get_result(&self.0)
                            .expect("Error loading user's status.");

                        if result > 0 { true } else { false }
                    },
                    highest_score: if msg.user_id.is_none() { None } else {
                        let result = status::table
                            .filter(status::score.is_not_null())
                            .filter(status::problem_region.eq(msg.region.clone()))
                            .filter(status::problem_id.eq(p_id))
                            .filter(status::owner_id.eq(msg.user_id.unwrap()))
                            .select(status::score)
                            .order_by(status::score.desc())
                            .first::<Option<f64>>(&self.0);

                        match result {
                            Err(_) => { None },
                            Ok(highest_score) => highest_score,
                        }
                    },
                }
            );
            page_problem_count += 1;
            catalog.total_count += 1;
        }
        catalog.page_count = current_page_number + 1;
        Ok(catalog)
    }
}

pub async fn get_problem_catalog_service (
    data: web::Data<DBState>,
    msg: GetProblemCatalogForm,
    id: Identity,
) -> ServiceResult<ProblemCatalog> {

    let user_id = if id.identity().is_some() {
        Some(atoi::<i32>(id.identity().unwrap().as_bytes()).unwrap())
    } else { None };

    let db_result = data.db.send(
        GetProblemCatalogMessage {
            user_id: user_id,
            region: msg.region,
            elements_per_page: msg.elements_per_page,
            title: msg.title,
            tags: msg.tags,
            difficulty: msg.difficulty,
        }
    ).await;

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