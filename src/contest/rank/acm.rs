use crate::{
    utils::time::get_cur_naive_date_time,
    database::*,
    errors::{ServiceError, ServiceResult},
    status::model::Status,
    contest::model::Contest,
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct UserPreview {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct ACMSolutionPreview {
    pub problem_region: String,
    pub problem_id: i32,
    pub try_times: i32,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct ACMRankColume {
    pub is_unrated: bool,
    pub rank: Option<i32>,
    pub user_previews: UserPreview,
    pub total_accepted: i32,
    pub total_penalty: i32,
    pub solution_previews: Vec<ACMSolutionPreview>
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct ACMRank {
    pub total_count: i32,
    pub columes: Vec<Vec<ACMRankColume>>,
    pub page_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetACMRankMessage {
    pub region: String,
    pub columes_per_page: Option<i32>,
}

impl Message for GetACMRankMessage {
    type Result = Result<ACMRank, String>;
}

impl Handler<GetACMRankMessage> for DbExecutor {
    type Result = Result<ACMRank, String>;
    
    fn handle(&mut self, msg: GetACMRankMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::{ status, contest_register_lists, users, contests, problems, regions };

        // check judge type
        if regions::table.filter(regions::name.eq(msg.region.clone()))
            .select(regions::judge_type)
            .first::<Option<String>>(&self.0)
            .expect("Error while check judge type.") != Some(String::from("ACM")) {
            return Err(String::from("Contest is not ACM type."))
        }

        // get contesters
        let contesters = contest_register_lists::table
            .filter(contest_register_lists::contest_region.eq(msg.region.clone()))
            .inner_join(users::table.on(contest_register_lists::user_id.eq(users::id)))
            .select((users::id, users::username, contest_register_lists::is_unrated))
            .load::<(i32, String, bool)>(&self.0)
            .expect("Error while loading contesters.");

        // get contest info
        let contest_info = contests::table
            .filter(contests::region.eq(msg.region.clone()))
            .first::<Contest>(&self.0)
            .expect("Error while loading contest.");
        
        // get contest problem list
        let contest_problems = problems::table.filter(problems::region.eq(msg.region.clone()))
            .select((problems::id, problems::region))
            .order_by(problems::id.asc())
            .load::<(i32, String)>(&self.0)
            .expect("Error while loading contest problems.");

        let mut rank_vec: Vec<ACMRankColume> = Vec::new();
        let mut problem_count = 0;
        for (user_id, username, is_unrated) in contesters {
            // prepare colume
            let mut personal_colume = ACMRankColume {
                is_unrated: is_unrated,
                rank: None,
                user_previews: UserPreview {
                    id: user_id,
                    username: username,
                },
                total_accepted: 0,
                total_penalty: 0,
                solution_previews: {
                    let mut solution_previews = Vec::new();
                    for (problem_id, problem_region) in contest_problems.clone() {
                        problem_count += 1;
                        solution_previews.push(
                            ACMSolutionPreview {
                                problem_region: problem_region,
                                problem_id: problem_id,
                                try_times: 0,
                                state: String::from("Untried")
                            }
                        )
                    }
                    solution_previews
                },
            };

            let related_status = status::table
                .filter(status::submit_time.ge(contest_info.start_time))
                .filter(status::submit_time.le(contest_info.end_time))
                .filter(status::judge_type.eq(String::from("ACM")))
                .filter(status::problem_region.eq(msg.region.clone()))
                .filter(status::owner_id.eq(user_id))
                .order_by(status::problem_id.asc())
                .then_order_by(status::submit_time.asc())
                .load::<Status>(&self.0)
                .expect("Error while loading contester's status");
            
            let mut solution_index;
            for status in related_status {
                solution_index = 0;
                while personal_colume.solution_previews[solution_index].problem_id != status.problem_id {
                    solution_index += 1;
                }

                let cur_time = get_cur_naive_date_time();    
                // if effective submit
                if status.result == Some(String::from("Accepted")) || status.result == Some(String::from("Unaccepted")) {
                    if personal_colume.solution_previews[solution_index].state != String::from("Accepted") {
                        personal_colume.solution_previews[solution_index].try_times += 1;
                        // if set seal time, check
                        if let Some(seal_before_end) = contest_info.seal_before_end {
                            // if in sealed time
                            if cur_time < contest_info.end_time && cur_time.timestamp() + seal_before_end as i64 >= contest_info.end_time.timestamp() {                                // if submit happened in sealed time
                                if status.submit_time.timestamp() + seal_before_end as i64 > contest_info.end_time.timestamp() {
                                    personal_colume.solution_previews[solution_index].state = String::from("Sealed");
                                    continue;
                                }
                            }
                        }
                        // not in sealed time or sealed time didn't set
                        if personal_colume.solution_previews[solution_index].state != String::from("Sealed") {
                            if status.result == Some(String::from("Accepted")) {
                                personal_colume.solution_previews[solution_index].try_times -= 1;
                                personal_colume.solution_previews[solution_index].state = String::from("Accepted");
                                personal_colume.total_penalty += 20*60*personal_colume.solution_previews[solution_index].try_times;
                                personal_colume.total_accepted += 1;
                            }
                            if status.result == Some(String::from("Unaccepted")) {
                                personal_colume.solution_previews[solution_index].state = String::from("Unaccepted");
                            }
                        }
                    }
                }
            }
            rank_vec.push(personal_colume);
        }
        // sort
        let slice = rank_vec.as_mut_slice();
        slice.sort_by(|colume_a, colume_b| {
            if colume_a.total_accepted != colume_b.total_accepted {
                colume_a.total_accepted.cmp(&colume_b.total_accepted).reverse()
            } else {
                colume_a.total_penalty.cmp(&colume_b.total_penalty)
            }
        });
        // assgin rank
        let mut rank_count = 0;
        let mut last_total_accepted = problem_count + 1;
        let mut last_total_penalty = 0;
        for colume in slice.iter_mut() {
            if colume.is_unrated { continue; }
            if colume.total_accepted < last_total_accepted { rank_count += 1; }
            else if colume.total_penalty > last_total_penalty { rank_count += 1; }

            last_total_accepted = colume.total_accepted;
            last_total_penalty = colume.total_penalty;
            colume.rank = Some(rank_count);
        }
        rank_vec = slice.to_vec();        

        let mut result = ACMRank {
            total_count: 0,
            columes: Vec::new(),
            page_count: 0,
        };
        let mut current_page_number = 0;
        let mut page_colume_count = 0;
        result.columes.push(Vec::new());
        for colume in rank_vec {
            if !msg.columes_per_page.is_none() && msg.columes_per_page.unwrap() == page_colume_count {
                result.columes.push(Vec::new());
                current_page_number += 1;
                page_colume_count = 0;
            }
            result.columes[current_page_number as usize].push(colume);
            page_colume_count += 1;
            result.total_count += 1;
        }

        Ok(result)
    }
}

pub async fn get_acm_rank_service (
    data: web::Data<DBState>,
    msg: GetACMRankMessage,
    _id: Identity,
) -> ServiceResult<ACMRank> {
    let db_result = data.db.send(msg).await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(catalog) => Ok(catalog),
            }
        }
    }
}
