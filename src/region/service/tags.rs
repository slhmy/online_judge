use crate::{
    database::*,
    errors::{ServiceError, ServiceResult},
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;

#[derive(Debug, Clone, Deserialize)]
pub struct GetRegionTagsMessage {
    pub region: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, juniper::GraphQLObject)]
pub struct OutTags {
    elements: Vec<String>,
}

impl Message for GetRegionTagsMessage {
    type Result = Result<OutTags, String>;
}

impl Handler<GetRegionTagsMessage> for DbExecutor {
    type Result = Result<OutTags, String>;
    
    fn handle(&mut self, msg: GetRegionTagsMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::problems::dsl::*;

        let result = problems.filter(region.eq(msg.region.clone()))
            .select(tags)
            .load::<Vec<String>>(&self.0)
            .expect("Error loading problems' tags.");

        let mut output = Vec::new();
        for problem_tags in result {
            for tag in problem_tags {
                output.push(tag);
            }
        }
        
        Ok(OutTags{elements: output})
    }
}

pub async fn get_region_tags_service (
    data: web::Data<DBState>,
    msg: GetRegionTagsMessage,
    _id: Identity,
) -> ServiceResult<OutTags> {

    let db_result = data.db.send(
        msg
    ).await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(tags) => Ok(tags),
            }
        }
    }
}