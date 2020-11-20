#[derive(Debug, Clone, Deserialize, Queryable)]
pub struct Problem {
    pub id: i32,
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
    pub submit_times: i32,
    pub accept_times: i32,
    pub default_max_cpu_time: i32,
    pub default_max_memory: i32,
    pub test_case: Option<String>,
    pub max_score: i32,
    pub opaque_output: bool,
}

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
    pub id: i32,
    pub region: String,
    pub title: String,
    pub default_max_cpu_time: i32,
    pub default_max_memory: i32,
    pub max_score: i32,
    pub problem: ProblemContext,
    pub tags: Option<Vec<String>>,
    pub sources: Option<Vec<String>>,
    pub difficulty: String,
    pub accept_times: i32,
    pub submit_times: i32,
    pub accept_rate: f64,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct DeleteProblemResult {
    pub result: String,
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
            ..
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