#[derive(Debug, Clone, Deserialize, Queryable)]
pub struct TestCase {
    pub name: String,
    pub is_spj: bool,
}