pub mod service;
pub mod model;
pub mod handler;
pub mod rank;

use actix_web::web;
use handler::*;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/contest")
            .service(web::resource("/get_contest").route(web::post().to(get_contest)))
            .service(web::resource("/new_contest").route(web::post().to(new_contest)))
            .service(web::resource("/delete_contest").route(web::post().to(delete_contest)))
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/get_catalog").route(web::post().to(get_contest_catalog)))
            .service(web::resource("/get_acm_rank").route(web::post().to(get_acm_rank)))
            .service(web::resource("/unregister").route(web::post().to(unregister)))
    );
}