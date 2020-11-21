use crate::{
    schema::problems,
    schema::status,
    schema::regions,
    database::*,
    utils::model::DeleteResult,
};
use diesel::prelude::*;
use actix::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRegionMessage {
    pub name: String,
}

impl Message for DeleteRegionMessage {
    type Result = Result<DeleteResult, String>;
}

impl Handler<DeleteRegionMessage> for DbExecutor {
    type Result = Result<DeleteResult, String>;

    fn handle(&mut self, msg: DeleteRegionMessage, _: &mut Self::Context) -> Self::Result {

        match diesel::delete(status::table
            .filter(status::problem_region.eq(msg.name.clone())))
            .execute(&self.0)
        {
            Err(_) => { Err("Error while deleting problem related status.".to_owned()) },
            Ok(_) => {
                match diesel::delete(problems::table
                    .filter(problems::region.eq(msg.name.clone())))
                    .execute(&self.0)
                {
                    Err(_) => { Err("Error while deleting problem.".to_owned()) },
                    Ok(_) => { 
                        match diesel::delete(regions::table
                            .filter(regions::name.eq(msg.name.clone())))
                            .execute(&self.0)
                        {
                            Err(_) => { Err("Error while deleting region.".to_owned()) },
                            Ok(_) => { Ok(DeleteResult {
                                result: "success".to_owned(),
                            }) }
                        }   
                    }
                }
            }
        }
    }
}
