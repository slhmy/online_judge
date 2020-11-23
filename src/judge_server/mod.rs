pub mod service;
pub mod handler;
pub mod config;
pub mod model;
pub mod utils;

use handler::*;
use service::{
    heartbeat::handle_heartbeat,
    // ping::ping_judge_server,
};
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/judge_server")
            .service(web::resource("/heartbeat").route(web::post().to(handle_heartbeat)))
            //.service(web::resource("/ping").route(web::post().to(ping_judge_server)))
            .service(web::resource("/submit").route(web::post().to(submit)))
            .service(web::resource("/info").route(web::post().to(get_server_info)))
            .service(web::resource("/get_file").route(web::post().to(get_file)))
    );
}