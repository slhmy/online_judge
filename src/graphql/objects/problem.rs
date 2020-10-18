use juniper::GraphQLObject;
use super::user::*;

#[derive(GraphQLObject)]
/// Inner object for 'Problem'.
/// Consider Srouce can be from local-site event or from the outside-site.
/// So we provide link for sources.
pub struct ProblemSrouce {
    pub name: String,
    pub link: String,
}

#[derive(GraphQLObject)]
/// Inner object for 'Problem'.
/// Consider Srouce can be from local-site event or from the outside-site.
/// You can use tag_id to quickly visit tag's detail information.
pub struct ProblemTag{
    pub name: String,
    pub tag_id: i32,
}

#[derive(GraphQLObject)]
/// Inner object for 'ProblemContent'
/// It is used to store several examples for the problem.
pub struct Example {
    pub input_example: String,
    pub output_example: String,
}

#[derive(GraphQLObject)]
/// Inner object for 'Problem'.
/// 'ProblemContent' contains all useful information describing the problem.
pub struct ProblemContent {
    pub description: String,
    pub input: String,
    pub output: String,
    pub examples: Vec<Example>,
    pub hint: Option<String>,
}

#[derive(GraphQLObject)]
/// Basic object for problem.
pub struct Problem {
    /// Unique identification for each problem.
    pub id: i32,
    pub content: ProblemContent,
    /// Shows where the problem are used.
    pub sources: Vec<ProblemSrouce>,
    /// Algorithm tags
    pub tags: Vec<ProblemTag>,
    /// Set region for status.
    /// This is mainly for regional query.
    /// For example, each contest should have its own region.
    pub region: String,
    /// Default lowest identity which is needed to visit this problem.
    pub lowest_user_identity: UserIdentity,
    /// Special whitelist for admission.
    /// Mainly for teacher administration.
    pub whitelist: Vec<String>,
}