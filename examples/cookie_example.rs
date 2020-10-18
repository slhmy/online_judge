use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use std::io;

use actix_web::{web, App, HttpResponse, HttpServer};


async fn index(id: Identity) -> String {
    // access request identity
    if let Some(id) = id.identity() {
        format!("Welcome! {}", id)
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

async fn login(id: Identity) -> HttpResponse {
    if let Some(_user_id) = id.identity() {
        id.forget();
        id.remember("User1".to_owned()); // <- remember identity
    }
    id.remember("User1".to_owned());
    HttpResponse::Ok().finish()
}

async fn logout(id: Identity) -> HttpResponse {
    id.forget();                      // <- remove identity
    HttpResponse::Ok().finish()
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new().wrap(IdentityService::new(
        // <- create identity middleware
        CookieIdentityPolicy::new(&[0; 32])    // <- create cookie identity policy
              .name("auth-cookie")
              .max_age(10)
              .secure(false)))
        .service(web::resource("/index.html").to(index))
        .service(web::resource("/login.html").to(login))
        .service(web::resource("/logout.html").to(logout))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}