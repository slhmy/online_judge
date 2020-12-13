pub struct MutationRoot;
// use juniper::FieldResult;
use futures::executor;
use super::Context;
use crate::judge_server::service::submit::{ submit_service, SubmitResult };
use crate::region::service::new::NewRegionMessage;
use crate::contest::{
    service::{
        new::{ new_contest_service, NewContestMessage },
        delete:: { delete_contest_service, DeleteContestMessage },
        register::register_service,
        unregister::{ unregister_service, UnregisterForm },
    },
    model::{ OutContest, RegisterInfo },
};
use crate::problem::{
    service::{
        new::{ new_problem_service, NewProblemMessage },
        update::{ update_problem_service, UpdateProblemMessage },
        delete::{ delete_problem_service, DeleteProblemMessage },
    },
    model::OutProblem,
};
use crate::test_case::{
    service::{
        delete::{ delete_test_case_service, DeleteTestCaseMessage },
    },
};
use crate::utils::model::DeleteResult;
use crate::errors::ServiceResult;

#[juniper::object(Context = Context)]
/// This is the root for all kinds of mutations, if you want to change something in the schema, use this object
impl MutationRoot {

    fn submit(
        context: &Context,
        problem_id: i32,
        region: String,
        src: String,
        language: String,
        judge_type: String,
        output: bool,
    ) -> ServiceResult<SubmitResult> {
        executor::block_on(submit_service(
            context.db.clone(),
            context.jm.clone(),
            problem_id,
            region,
            src,
            language,
            judge_type,
            output,
            context.id.clone()))
    }

    fn new_contest(
        context: &Context,
        region: String,
        name: String, 
        start_time: String,
        end_time: String,
        seal_before_end: Option<i32>,
        register_end_time: Option<String>,
        judge_type: String,
        password: Option<String>,
    ) -> ServiceResult<OutContest> {
        executor::block_on(new_contest_service(
            context.db.clone(),
            NewRegionMessage {
                name: region.clone(),
                password: password,
                self_type: "contest".to_owned(),
                judge_type: judge_type,
            },
            NewContestMessage {
                region: region,
                name: name, 
                start_time: start_time,
                end_time: end_time,
                seal_before_end: seal_before_end,
                register_end_time: register_end_time,
            },
            context.id.clone(),
        ))
    }

    fn delete_contest(
        context: &Context,
        region_name: String,
    ) -> ServiceResult<DeleteResult> {
        executor::block_on(delete_contest_service(
            context.db.clone(),
            DeleteContestMessage {
                region_name,
            },
            context.id.clone(),
        ))
    }

    fn register_contest(
        context: &Context,
        contest_region: String,
        is_unrated: bool,
        password: Option<String>,
    ) -> ServiceResult<RegisterInfo> {
        executor::block_on(register_service(
            context.db.clone(),
            contest_region,
            is_unrated,
            password,
            context.id.clone(),
        ))
    }

    fn unregister_contest(
        context: &Context,
        region: String,
    ) -> ServiceResult<DeleteResult> {
        executor::block_on(unregister_service(
            context.db.clone(),
            UnregisterForm{ region },
            context.id.clone(),
        ))
    }

    fn new_problem(
        context: &Context,
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
        default_max_cpu_time: i32,
        default_max_memory: i32,
        test_case: Option<String>,
        max_score: i32,
        opaque_output: bool,
    ) -> ServiceResult<OutProblem> {
        executor::block_on(new_problem_service(
            context.db.clone(),
            NewProblemMessage {
                id: id,
                region: region,
                title: title,
                description: description,
                input_explain: input_explain,
                output_explain: output_explain,
                input_examples: input_examples,
                output_examples: output_examples,
                hint: hint,
                tags: tags,
                sources: sources,
                difficulty: difficulty,
                default_max_cpu_time: default_max_cpu_time,
                default_max_memory: default_max_memory,
                test_case: test_case,
                max_score: max_score,
                opaque_output: opaque_output,
            },
            context.id.clone(),
        ))
    }

    fn update_problem(
        context: &Context,
        id: i32,
        region: String,
        new_id: Option<i32>,
        new_title: Option<String>,
        new_description: Option<String>,
        new_input_explain: Option<String>,
        new_output_explain: Option<String>,
        new_input_examples: Option<Vec<String>>,
        new_output_examples: Option<Vec<String>>,
        new_hint: Option<String>,
        new_tags: Option<Vec<String>>,
        new_sources: Option<Vec<String>>,
        new_difficulty: Option<String>,
        new_default_max_cpu_time: Option<i32>,
        new_default_max_memory: Option<i32>,
        new_test_case: Option<String>,
        new_max_score: Option<i32>,
        new_opaque_output: Option<bool>,
    ) -> ServiceResult<OutProblem> {
        executor::block_on(update_problem_service(
            context.db.clone(),
            UpdateProblemMessage {
                id: id,
                region: region,
                new_id: new_id,
                new_title: new_title,
                new_description: new_description,
                new_input_explain: new_input_explain,
                new_output_explain: new_output_explain,
                new_input_examples: new_input_examples,
                new_output_examples: new_output_examples,
                new_hint: new_hint,
                new_tags: new_tags,
                new_sources: new_sources,
                new_difficulty: new_difficulty,
                new_default_max_cpu_time: new_default_max_cpu_time,
                new_default_max_memory: new_default_max_memory,
                new_test_case: new_test_case,
                new_max_score: new_max_score,
                new_opaque_output: new_opaque_output,
            },
            context.id.clone(),
        ))
    }

    fn delete_problem(
        context: &Context,
        id: i32,
        region: String,
    ) -> ServiceResult<DeleteResult> {
        executor::block_on(delete_problem_service(
            context.db.clone(),
            DeleteProblemMessage {
                id: id,
                region: region,
            },
            context.id.clone(),
        ))
    }

    fn delete_test_case(
        context: &Context,
        name: String,
    ) -> ServiceResult<DeleteResult> {
        executor::block_on(delete_test_case_service(
            context.db.clone(),
            DeleteTestCaseMessage {
                name: name,
            },
            context.id.clone(),
        ))
    }
}