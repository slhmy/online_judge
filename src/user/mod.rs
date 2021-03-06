pub mod model;
pub mod service;
mod handler;

use actix_web::web;
use service::{
    login::*,
    manage::*,
    register::*,
    me::*,
};
use handler::*;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/get_verification_code").route(web::post().to(get_verification_code)))
            .service(web::resource("/catalog").route(web::post().to(get_user_catalog)))
            .service(web::resource("/quick_login").route(web::post().to(quick_login)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/logout").route(web::get().to(logout)))
            .service(
                web::scope("/manage")
                    .service(web::resource("/get_all_users").route(web::get().to(get_all_users)))
                    .service(web::resource("/change_info").route(web::post().to(change_info))) 
                    .service(web::resource("/delete_user").route(web::post().to(delete_user)))
                    .service(web::resource("/auto_register").route(web::post().to(auto_register)))
            )
            .service(web::resource("/me").route(web::get().to(me))),
    );
}