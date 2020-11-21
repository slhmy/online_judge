#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct DeleteResult {
    pub result: String,
}