use crate::{
    user::model::*,
    database::*,
    errors::{ServiceError, ServiceResult},
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct UserCatalog {
    pub total_count: i32,
    pub elements: Vec<Vec<OutUser>>,
    pub page_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserCatalogMessage {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub job_number: Option<String>,
    pub elements_per_page: Option<i32>,
}

impl Message for GetUserCatalogMessage {
    type Result = Result<UserCatalog, String>;
}

impl Handler<GetUserCatalogMessage> for DbExecutor {
    type Result = Result<UserCatalog, String>;
    
    fn handle(&mut self, msg: GetUserCatalogMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let search_username = if msg.username.is_some() {
            let result = str::replace(&msg.username.unwrap(), " ", "%");
            Some("%".to_owned() + &result + "%")
        } else { None };

        let search_mobile = if msg.mobile.is_some() {
            let result = str::replace(&msg.mobile.unwrap(), " ", "%");
            Some("%".to_owned() + &result + "%")
        } else { None };

        let search_email = if msg.email.is_some() {
            let result = str::replace(&msg.email.unwrap(), " ", "%");
            Some("%".to_owned() + &result + "%")
        } else { None };

        let search_job_number = if msg.job_number.is_some() {
            let result = str::replace(&msg.job_number.unwrap(), " ", "%");
            Some("%".to_owned() + &result + "%")
        } else { None };

        let result = users
            .filter(id.nullable().eq(msg.id).or(msg.id.is_none()))
            .filter(username.ilike(search_username.clone().unwrap_or("".to_owned())).or(search_username.is_none()))
            .filter(mobile.ilike(search_mobile.clone().unwrap_or("".to_owned())).or(search_mobile.is_none()))
            .filter(email.ilike(search_email.clone().unwrap_or("".to_owned())).or(search_email.is_none()))
            .filter(job_number.ilike(search_job_number.clone().unwrap_or("".to_owned())).or(search_job_number.is_none()))
            .order_by(id.asc())
            .load::<User>(&self.0)
            .expect("Error loading users.");

        let mut catalog = UserCatalog {
            total_count: 0,
            elements: Vec::new(),
            page_count: 0,
        };

        let mut current_page_number = 0;
        let mut page_user_count = 0;
        catalog.elements.push(Vec::new());
        info!("{:?}", result);
        for user in result {
            if !msg.elements_per_page.is_none() && msg.elements_per_page.unwrap() == page_user_count {
                catalog.elements.push(Vec::new());
                current_page_number += 1;
                page_user_count = 0;
            }
            catalog.elements[current_page_number as usize].push(
                OutUser::from(user)
            );
            page_user_count += 1;
            catalog.total_count += 1;
        }
        catalog.page_count = current_page_number + 1;
        Ok(catalog)
    }
}

pub async fn get_user_catalog_service (
    data: web::Data<DBState>,
    msg: GetUserCatalogMessage,
    _id: Identity,
) -> ServiceResult<UserCatalog> {
    let db_result = data.db.send(msg).await;

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