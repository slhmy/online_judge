pub mod service;
pub mod handler;
pub mod config;
pub mod model;
pub mod utils;

use handler::judge;
use service::{
    heartbeat::handle_heartbeat,
    ping::ping_judge_server,
};
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/judge_server")
            .service(web::resource("/heartbeat").route(web::post().to(handle_heartbeat)))
            .service(web::resource("/ping").route(web::post().to(ping_judge_server)))
            .service(web::resource("/judge").route(web::post().to(judge)))
    );
}