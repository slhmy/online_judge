use actix::prelude::*;
use diesel::prelude::*;
use crate::statics::JUDGE_SERVER_INFOS;
use crate::statics::WAITING_QUEUE;
use crate::JudgeManager;
use crate::judge_manager::utils::{ 
    chooser::choose_judge_server,
    process::run_judge_client,
    result::get_judge_result,
};
use crate::utils::time::get_cur_naive_date_time;

#[derive(Debug, Clone, Deserialize)]
pub struct StartJudge();

impl Message for StartJudge {
    type Result = ();
}

impl Handler<StartJudge> for JudgeManager {
    type Result = ();
    
    fn handle(&mut self, _msg: StartJudge, _: &mut Self::Context) -> Self::Result {
        use crate::schema::status::dsl::*;

        let mut queue_size = {
            let lock = WAITING_QUEUE.read().unwrap();
            lock.len().clone()
        };
        while queue_size != 0 {
            let server = choose_judge_server();
            if server.is_none() { return (); }
            let (server_url, server_token) = server.unwrap();
            {
                let mut lock = JUDGE_SERVER_INFOS.write().unwrap();
                let mut server_info = lock.get(&server_url).unwrap().clone();
                server_info.task_number += 1;
                lock.insert(server_url.clone(), server_info);
            }
            let task_uuid = {
                let mut lock = WAITING_QUEUE.write().unwrap();
                lock.pop_front().clone().unwrap()
            };

            let cur_state = status
                .filter(id.eq(task_uuid))
                .select(state)
                .first::<String>(&self.0)
                .expect("Error loading setting_data from status.");

            if cur_state == "Waiting".to_owned() {
                let (judge_type_string, setting_string) = status
                    .filter(id.eq(task_uuid))
                    .select((judge_type, setting_data))
                    .first::<(String, String)>(&self.0)
                    .expect("Error loading setting_data from status.");

                let target = status.filter(id.eq(task_uuid));
                diesel::update(target)
                    .set((
                        state.eq("Pending".to_owned()),
                        start_pend_time.eq(Some(get_cur_naive_date_time())),
                    ))
                    .execute(&self.0).expect("Error changing status's state to Pending.");

                let result_string = run_judge_client(server_token, setting_string);
                info!("{}", result_string);

                let (op_result, op_score, op_err_reason) = get_judge_result(judge_type_string, result_string.clone());

                {
                    let mut lock = JUDGE_SERVER_INFOS.write().unwrap();
                    let mut server_info = lock.get(&server_url).unwrap().clone();
                    server_info.task_number -= 1;
                    lock.insert(server_url, server_info);
                }

                let target = status.filter(id.eq(task_uuid));
                diesel::update(target)
                    .set((
                        state.eq("Finished".to_owned()),
                        result.eq(op_result),
                        score.eq(op_score),
                        result_data.eq(Some(result_string)),
                        err_reason.eq(op_err_reason),
                        finish_time.eq(Some(get_cur_naive_date_time())),
                    ))
                    .execute(&self.0).expect("Error changing status's data.");
            }

            queue_size = {
                let lock = WAITING_QUEUE.read().unwrap();
                lock.len().clone()
            }; 
        }

        ()
    }
}