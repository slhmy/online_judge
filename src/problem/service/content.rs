use crate::{
    database::*,
    problem::model::{ Problem, OutProblem },
    errors::{ServiceError, ServiceResult},
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;

#[derive(Debug, Clone, Deserialize)]
pub struct GetProblemMessage {
    pub id: i32,
    pub region: String,
}

impl Message for GetProblemMessage {
    type Result = Result<OutProblem, String>;
}

impl Handler<GetProblemMessage> for DbExecutor {
    type Result = Result<OutProblem, String>;
    
    fn handle(&mut self, msg: GetProblemMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::problems::dsl::*;

        let result = problems.filter(region.eq(msg.region))
            .filter(id.eq(msg.id))
            .first::<Problem>(&self.0)
            .expect("Error loading problems.");

        Ok(OutProblem::from(result))
    }
}


pub async fn get_problem(
    data: web::Data<DBState>,
    id: i32,
    region: String,
    _id: Identity,
) -> ServiceResult<OutProblem> {
    let db_result = data.db.send(GetProblemMessage {
        id: id,
        region: region,
    }).await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(problem) => Ok(problem),
            }
        }
    }
}