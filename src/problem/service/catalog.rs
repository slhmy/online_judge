use crate::{
    database::*,
    errors::{ServiceError, ServiceResult},
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct ProblemCatalogElement {
    pub id: i32,
    pub title: String,
    pub tags: Option<Vec<String>>,
    pub difficulty: String,
    pub accept_times: i32,
    pub submit_times: i32,
    pub accept_rate: f64,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct ProblemCatalog {
    pub total_count: i32,
    pub elements: Vec<Vec<ProblemCatalogElement>>,
    pub page_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetProblemCatalogMessage {
    pub region: String,
    pub problems_per_page: Option<i32>,
}

impl Message for GetProblemCatalogMessage {
    type Result = Result<ProblemCatalog, String>;
}

impl Handler<GetProblemCatalogMessage> for DbExecutor {
    type Result = Result<ProblemCatalog, String>;
    
    fn handle(&mut self, msg: GetProblemCatalogMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::problems::dsl::*;

        let result = problems.filter(region.eq(msg.region))
            .select( (id, title, tags, difficulty, accept_times, submit_times) )
            .load::<(i32, String, Option<Vec<String>>, String, i32, i32)>(&self.0)
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
            if !msg.problems_per_page.is_none() && msg.problems_per_page.unwrap() == page_problem_count {
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
    region: String,
    problems_per_page: Option<i32>,
) -> ServiceResult<ProblemCatalog> {
    let db_result = data.db.send(GetProblemCatalogMessage {
        region: region,
        problems_per_page: problems_per_page,
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