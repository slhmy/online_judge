use crate::schema::test_cases;

#[derive(Debug, Clone, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "test_cases"]
pub struct TestCase {
    pub name: String,
    pub is_spj: bool,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct DeleteTestCaseResult {
    pub result: String,
}