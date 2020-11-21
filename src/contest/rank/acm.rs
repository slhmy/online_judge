#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct UserPreview {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct SolutionPreview {
    pub region: String,
    pub id: i32,
    pub try_times: i32,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct RankColume {
    pub rank: Option<i32>,
    pub user_previews: UserPreview,
    pub total_accepted: i32,
    pub total_penalty: i32,
    pub solution_previews: Vec<SolutionPreview>
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct Rank {
    pub columes: Vec<RankColume>
}