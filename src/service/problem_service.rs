use crate::graphql_schema::objects::{
    problem::*,
    user::*,
};

pub fn get_problem_by_id(id: i32) -> Problem {
    Problem {
        id: 1,
        content: ProblemContent {
            description: "This is the simplest a+b problem.".to_owned(),
            input: "Input two integers in two lines representing a and b.".to_owned(),
            output: "Ouput the calculation result.".to_owned(),
            examples: vec![Example{
                input_example: "1\n2\n".to_owned(),
                output_example: "3\n".to_owned(),
            }],
            hint: None,
        },
        sources: Vec::new(),
        tags: vec![ProblemTag {
            name: "Basic".to_owned(),
            tag_id: 1,
        }],
        region: "Global".to_owned(),
        lowest_user_identity: UserIdentity::Teacher,
        whitelist: vec!["slhmy".to_owned()],
    }
}