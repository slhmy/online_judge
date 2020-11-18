#[derive(Debug, Clone, Deserialize, Queryable)]
pub struct Region {
    pub name: String,
    pub need_pass: bool,
    pub salt: Option<String>,
    pub hash: Option<Vec<u8>>,
    pub self_type: String,
    pub judge_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Queryable)]
pub struct OutRegion {
    pub name: String,
    pub need_pass: bool,
    pub self_type: String,
    pub judge_type: Option<String>,
}

impl From<Region> for OutRegion {
    fn from(region: Region) -> Self {
        let Region {
            name,
            need_pass,
            salt: _,
            hash: _,
            self_type,
            judge_type,
        } = region;

        Self {
            name,
            need_pass,
            self_type,
            judge_type,
        }
    }
}