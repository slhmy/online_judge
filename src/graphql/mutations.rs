pub struct MutationRoot;
// use juniper::FieldResult;
use futures::executor;
use super::Context;
use crate::judge_server::service::submit::{ submit_service, SubmitResult };
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
}