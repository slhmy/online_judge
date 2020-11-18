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

#[derive(Debug, Clone, Deserialize, Insertable, Queryable)]
#[table_name = "problems"]
struct InsertableProblem {
    id: i32,
    region: String,
    title: String,
    description: Option<String>,
    input_explain: Option<String>,
    output_explain: Option<String>,
    input_examples: Option<Vec<String>>,
    output_examples: Option<Vec<String>>,
    hint: Option<String>,
    tags: Option<Vec<String>>,
    sources: Option<Vec<String>>,
    difficulty: String,
    submit_times: i32,
    accept_times: i32,
    default_max_cpu_time: i32,
    default_max_memory: i32,
    test_case: Option<String>,
    max_score: i32,
    opaque_output: bool,
}

impl Message for NewProblemMessage {
    type Result = Result<OutProblem, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewProblemMessage {
    pub id: i32,
    pub region: String,
    pub title: String,
    pub description: Option<String>,
    pub input_explain: Option<String>,
    pub output_explain: Option<String>,
    pub input_examples: Option<Vec<String>>,
    pub output_examples: Option<Vec<String>>,
    pub hint: Option<String>,
    pub tags: Option<Vec<String>>,
    pub sources: Option<Vec<String>>,
    pub difficulty: String,
    pub default_max_cpu_time: i32,
    pub default_max_memory: i32,
    pub test_case: Option<String>,
    pub max_score: i32,
    pub opaque_output: bool,
}

impl Handler<NewProblemMessage> for DbExecutor {
    type Result = Result<OutProblem, String>;

    fn handle(&mut self, msg: NewProblemMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::problems::dsl::problems;

        let result = diesel::insert_into(problems)
            .values(&InsertableProblem{
                id: msg.id,
                region: msg.region,
                title: msg.title,
                description: msg.description,
                input_explain: msg.input_explain,
                output_explain: msg.output_explain,
                input_examples: msg.input_examples,
                output_examples: msg.output_examples,
                hint: msg.hint,
                tags: msg.tags,
                sources: msg.sources,
                difficulty: msg.difficulty,
                submit_times: 0,
                accept_times: 0,
                default_max_cpu_time: msg.default_max_cpu_time,
                default_max_memory: msg.default_max_memory,
                test_case: msg.test_case,
                max_score: msg.max_score,
                opaque_output: msg.opaque_output,
            })
            .get_result::<Problem>(&self.0);

        match result {
            Err(_) => { Err("Error while creating new problem.".to_owned()) },
            Ok(inner_result) => { Ok(OutProblem::from(inner_result)) }
        }
    }
}

pub async fn new_problem_service(
    data: web::Data<DBState>,
    msg: NewProblemMessage,
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