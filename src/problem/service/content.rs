use crate::{
    database::*,
    problem::model::Problem,
    errors::{ServiceError, ServiceResult},
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct Example {
    input_example: String,
    output_example: String,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct ProblemContext {
    max_cpu_time: i32,
    max_memory: i32,
    description: Option<String>, 
    input_explain: Option<String>,
    output_explain: Option<String>,
    examples: Option<Vec<Example>>,
    hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct OutProblem {
    id: i32,
    region: String,
    title: String,
    default_max_cpu_time: i32,
    default_max_memory: i32,
    max_score: i32,
    problem: ProblemContext,
    tags: Option<Vec<String>>,
    sources: Option<Vec<String>>,
    difficulty: String,
    pub accept_times: i32,
    pub submit_times: i32,
    pub accept_rate: f64,
}

impl From<Problem> for OutProblem {
    fn from(problem: Problem) -> Self {
        let Problem {
            id,
            region,
            title,
            description,
            input_explain,
            output_explain,
            input_examples,
            output_examples,
            hint,
            tags,
            sources,
            difficulty,
            submit_times,
            accept_times,
            default_max_cpu_time,
            default_max_memory,
            test_case: _,
            max_score,
        } = problem;

        let examples = 
            if !input_examples.is_none() {
                let mut unwraped_examples = Vec::new();
                let unwraped_input_examples = input_examples.unwrap();
                let unwraped_output_examples = output_examples.unwrap();
                for i in 0..unwraped_input_examples.len().min(unwraped_output_examples.len()) {
                    let input_example = unwraped_input_examples[i].clone();
                    let output_example = unwraped_output_examples[i].clone();
                    unwraped_examples.push(Example { input_example, output_example })
                }
                Some(unwraped_examples)
            } else { None };
        OutProblem {
            id: id,
            region: region,
            title: title,
            default_max_cpu_time: default_max_cpu_time,
            default_max_memory: default_max_memory,
            max_score: max_score,
            problem: ProblemContext {
                max_cpu_time: default_max_cpu_time,
                max_memory: default_max_memory,
                description: description, 
                input_explain: input_explain,
                output_explain: output_explain,
                examples: examples,
                hint: hint,
            },
            tags: tags,
            sources: sources,
            difficulty: difficulty,
            accept_times: accept_times,
            submit_times: submit_times,
            accept_rate: if submit_times == 0 { 0.0 } 
            else { accept_times as f64 / submit_times as f64 },
        }
    }
}

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
    data: web::Data<State>,
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