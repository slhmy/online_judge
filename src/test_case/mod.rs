pub mod service;
pub mod model;
pub mod handler;
pub mod utils;

use actix_web::web;
use handler::*;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/test_case")
            .service(web::resource("/get_catalog").route(web::post().to(get_test_case_catalog)))
            .service(web::resource("/new_test_case/{test_case_name}/{is_spj}").route(web::get().to(new_test_case)))
            .service(web::resource("/update_test_case/{test_case_name}/{is_spj}").route(web::get().to(update_test_case)))
            .service(web::resource("/delete_test_case").route(web::post().to(delete_test_case)))
    );
}