pub mod model;
pub mod utils;
pub mod service;
pub mod handler;

use actix_web::web;
use handler::*;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/status")
            .service(web::resource("/get_catalog").route(web::post().to(get_status_catalog)))
    );
}