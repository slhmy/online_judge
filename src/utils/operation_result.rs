#[derive(Debug, Clone, Serialize)]
pub struct OperationResult {
    pub result_en: Option<String>,
    pub msg_en: Option<String>,
    pub result_cn: Option<String>,
    pub msg_cn: Option<String>,
}