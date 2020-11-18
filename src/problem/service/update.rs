use crate::{
    schema::problems,
    database::*,
    problem::model::{ Problem, OutProblem },
    errors::{ ServiceError, ServiceResult },
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;

#[derive(Debug, Clone, Deserialize, AsChangeset)]
#[table_name = "problems"]
#[primary_key(id, region)] 
struct ProblemChange {
    title: Option<String>,
    description: Option<String>,
    input_explain: Option<String>,
    output_explain: Option<String>,
    input_examples: Option<Vec<String>>,
    output_examples: Option<Vec<String>>,
    hint: Option<String>,
    tags: Option<Vec<String>>,
    sources: Option<Vec<String>>,
    difficulty: Option<String>,
    submit_times: Option<i32>,
    accept_times: Option<i32>,
    default_max_cpu_time: Option<i32>,
    default_max_memory: Option<i32>,
    test_case: Option<String>,
    max_score: Option<i32>,
    opaque_output: Option<bool>,
}

impl Message for UpdateProblemMessage {
    type Result = Result<OutProblem, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProblemMessage {
    pub id: i32,
    pub region: String,
    pub new_id: Option<i32>,
    pub new_title: Option<String>,
    pub new_description: Option<String>,
    pub new_input_explain: Option<String>,
    pub new_output_explain: Option<String>,
    pub new_input_examples: Option<Vec<String>>,
    pub new_output_examples: Option<Vec<String>>,
    pub new_hint: Option<String>,
    pub new_tags: Option<Vec<String>>,
    pub new_sources: Option<Vec<String>>,
    pub new_difficulty: Option<String>,
    pub new_default_max_cpu_time: Option<i32>,
    pub new_default_max_memory: Option<i32>,
    pub new_test_case: Option<String>,
    pub new_max_score: Option<i32>,
    pub new_opaque_output: Option<bool>,
}

impl Handler<UpdateProblemMessage> for DbExecutor {
    type Result = Result<OutProblem, String>;

    fn handle(&mut self, msg: UpdateProblemMessage, _: &mut Self::Context) -> Self::Result {

        let target_id = 
            if msg.new_id.is_some() {
                let result = diesel::update(problems::table)
                    .filter(problems::region.eq(msg.region.clone()))
                    .filter(problems::id.eq(msg.id))
                    .set(problems::id.eq(msg.new_id.unwrap()))
                    .execute(&self.0);
                
                match result {
                    Err(_) => { return Err("Error while updating problem's id.\nThis is may because problem already have related elements or id has been used.".to_owned()); },
                    Ok(_) => {}
                }
                
                msg.new_id.unwrap()
            } else {
                msg.id
            };
        

        let result = diesel::update(problems::table)
            .filter(problems::region.eq(msg.region))
            .filter(problems::id.eq(target_id))
            .set(&ProblemChange{
                title: msg.new_title,
                description: msg.new_description,
                input_explain: msg.new_input_explain,
                output_explain: msg.new_output_explain,
                input_examples: msg.new_input_examples,
                output_examples: msg.new_output_examples,
                hint: msg.new_hint,
                tags: msg.new_tags,
                sources: msg.new_sources,
                difficulty: msg.new_difficulty,
                submit_times: None,
                accept_times: None,
                default_max_cpu_time: msg.new_default_max_cpu_time,
                default_max_memory: msg.new_default_max_memory,
                test_case: msg.new_test_case,
                max_score: msg.new_max_score,
                opaque_output: msg.new_opaque_output,
            })
            .get_result::<Problem>(&self.0);

        match result {
            Err(_) => { Err("Error while updating problem.".to_owned()) },
            Ok(inner_result) => { Ok(OutProblem::from(inner_result)) }
        }
    }
}

pub async fn update_problem_service(
    data: web::Data<DBState>,
    msg: UpdateProblemMessage,
    _id: Identity,
) -> ServiceResult<OutProblem> {
    let db_result = data.db.send(msg).await;

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