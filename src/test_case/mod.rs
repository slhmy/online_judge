pub mod service;
pub mod model;
pub mod handler;

use actix_web::web;
use handler::*;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/test_case")
            .service(web::resource("/get_catalog").route(web::post().to(get_test_case_catalog)))
    );
}