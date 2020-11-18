use chrono::*;

#[derive(Debug, Clone, Deserialize, Queryable)]
pub struct Contest {
    pub region: String,
    pub name: String,
    pub state: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub seal_before_end: Option<i32>,
    pub register_end_time: NaiveDateTime,
    pub final_rank: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct OutContest {
    pub region: String,
    pub name: String,
    pub state: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub seal_before_end: Option<i32>,
    pub register_end_time: NaiveDateTime,
}

impl From<Contest> for OutContest {
    fn from(contest: Contest) -> Self {
        let Contest {
            region,
            name,
            state,
            start_time,
            end_time,
            seal_before_end,
            register_end_time,
            ..
        } = contest;

        Self {
            region,
            name,
            state,
            start_time,
            end_time,
            seal_before_end,
            register_end_time,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct RegisterInfo {
    pub contest_region: String,
    pub user_id: i32,
    pub is_unrated: bool,
}