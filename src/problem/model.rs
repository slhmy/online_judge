#[derive(Debug, Clone, Deserialize, Queryable)]
pub struct Problem {
    pub id: usize,
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
    pub submit_times: usize,
    pub accept_times: usize,
}