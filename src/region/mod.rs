pub mod service;
pub mod model;
pub mod handler;

use actix_web::web;
use handler::*;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/region")
            .service(web::resource("/tags").route(web::post().to(get_tags)))
            .service(web::resource("/info").route(web::post().to(get_info)))
    );
}