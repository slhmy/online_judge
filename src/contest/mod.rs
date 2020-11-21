pub mod service;
pub mod model;
pub mod handler;

use actix_web::web;
use handler::*;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/contest")
            .service(web::resource("/new_contest").route(web::post().to(new_contest)))
            .service(web::resource("/delete_contest").route(web::post().to(delete_contest)))
            .service(web::resource("/register").route(web::post().to(register)))
    );
}