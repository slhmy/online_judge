use crate::{
    database::*,
    errors::{ServiceError, ServiceResult},
    contest::model::*,
    utils::time::get_cur_naive_date_time,
    region::model::*,
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;
use atoi::atoi;
use chrono::*;

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct ContestCatalogElement {
    pub region: String,
    pub name: String,
    pub state: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub seal_before_end: Option<i32>,
    pub register_end_time: NaiveDateTime,
    pub is_registered: bool,
    pub need_pass: bool,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct ContestCatalog {
    pub total_count: i32,
    pub elements: Vec<Vec<ContestCatalogElement>>,
    pub page_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetContestCatalogForm {
    pub name: Option<String>,
    pub elements_per_page: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetContestCatalogMessage {
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub elements_per_page: Option<i32>,
}

impl Message for GetContestCatalogMessage {
    type Result = Result<ContestCatalog, String>;
}

impl Handler<GetContestCatalogMessage> for DbExecutor {
    type Result = Result<ContestCatalog, String>;
    
    fn handle(&mut self, msg: GetContestCatalogMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::contests::dsl::*;
        use crate::schema::contests;
        use crate::schema::contest_register_lists;
        use crate::schema::regions;
        use diesel::dsl::*;

        let search_name = if msg.name.is_some() {
            let result = str::replace(&msg.name.unwrap(), " ", "%");
            Some("%".to_owned() + &result + "%")
        } else { None };

        let result = contests
            .filter(name.ilike(search_name.clone().unwrap_or("".to_owned())).or(search_name.is_none()))
            .order_by(start_time.desc())
            .load::<Contest>(&self.0)
            .expect("Error loading contests.");

        let mut catalog = ContestCatalog {
            total_count: 0,
            elements: Vec::new(),
            page_count: 0,
        };

        let mut current_page_number = 0;
        let mut page_element_count = 0;
        catalog.elements.push(Vec::new());
        info!("{:?}", result);
        for contest in result {
            if !msg.elements_per_page.is_none() && msg.elements_per_page.unwrap() == page_element_count {
                catalog.elements.push(Vec::new());
                current_page_number += 1;
                page_element_count = 0;
            }

            let is_registered = if msg.user_id.is_some() {
                match contest_register_lists::table
                    .filter(contest_register_lists::user_id.eq(msg.user_id.unwrap()))
                    .filter(contest_register_lists::contest_region.eq(contest.region.clone()))
                    .select(count_star())
                    .first::<i64>(&self.0) {
                    Err(_) => false,
                    Ok(count) => { if count >= 1 { true } else { false } },
                }
            } else { false };

            let cur_time = get_cur_naive_date_time();
            let supposed_state = {
                if cur_time < contest.start_time { String::from("Preparing") }
                else if cur_time > contest.end_time { String::from("Ended") }
                else { String::from("Running") }
            };

            catalog.elements[current_page_number as usize].push(
                ContestCatalogElement {
                    region: contest.region.clone(),
                    name: contest.name,
                    state: {
                        if supposed_state == contest.state { contest.state }
                        else {
                            let target = contests::table
                                .filter(contests::region.eq(contest.region.clone()));
                            diesel::update(target)
                                .set(contests::state.eq(supposed_state.clone()))
                                .execute(&self.0).expect("Error changing status's state to Pending.");
                            supposed_state
                        }
                    },
                    start_time: contest.start_time,
                    end_time: contest.end_time,
                    seal_before_end: contest.seal_before_end,
                    register_end_time: contest.register_end_time,
                    is_registered: is_registered,
                    need_pass: {
                        let cur_region = regions::table
                            .filter(regions::name.eq(contest.region))
                            .first::<Region>(&self.0).expect("Error getting region");
                        cur_region.need_pass
                    }
                }
            );
            page_element_count += 1;
            catalog.total_count += 1;
        }
        catalog.page_count = current_page_number + 1;
        Ok(catalog)
    }
}

pub async fn get_contest_catalog_service (
    data: web::Data<DBState>,
    msg: GetContestCatalogForm,
    id: Identity,
) -> ServiceResult<ContestCatalog> {

    let user_id = if id.identity().is_some() {
        Some(atoi::<i32>(id.identity().unwrap().as_bytes()).unwrap())
    } else { None };

    let db_result = data.db.send(
        GetContestCatalogMessage {
            user_id: user_id,
            name: msg.name,
            elements_per_page: msg.elements_per_page,
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