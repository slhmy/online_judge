pub mod model;
pub mod service;
pub mod handler;

use actix_web::web;
use handler::*;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/problem")
            .service(web::resource("/get_catalog").route(web::post().to(get_problem_catalog)))
            .service(web::resource("/get_problem").route(web::post().to(get_problem)))
            .service(web::resource("/new_problem").route(web::post().to(new_problem)))
    );
}