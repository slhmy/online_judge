mod schema;
mod judge_manager;
mod database;
mod user;
mod problem;
mod status;
mod graphql;
mod judge_server;
mod utils;
mod errors;
mod statics;
mod region;
mod contest;
mod test_case;

#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

extern crate regex;
extern crate dotenv;
extern crate env_logger;
extern crate serde_json;
extern crate pretty_env_logger;

use actix_web::{ 
    App,
    middleware, 
    HttpServer,
    //cookie::SameSite,
};
use actix_identity::{
    CookieIdentityPolicy, 
    IdentityService,
};
use actix_cors::Cors;
use crate::{
    graphql::schema as graphql_schema,
    database::*,
    judge_manager::*,
};
use time::Duration;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info, actix_web=info");
    env_logger::init();

    // Create schema
    let db_addr = create_db_executor();
    let jm_addr = create_judge_manager();

    // Create Juniper schema
    let graphql_schema = std::sync::Arc::new(graphql_schema::create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(DBState { db: db_addr.clone() })
            .data(JMState { jm: jm_addr.clone() })
            .data(graphql_schema.clone())
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .supports_credentials()
                    .finish()
                )
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                // <- create identity middleware
                CookieIdentityPolicy::new(&[0; 32])    // <- create cookie identity policy
                      .name("auth-cookie")
                      //.same_site(SameSite::None)
                      .path("/")
                      .http_only(false)
                      .max_age(1800)    
                      .visit_deadline(Duration::minutes(30))
                      .secure(false)))
            .configure(graphql::route)
            .configure(user::route)
            .configure(problem::route)
            .configure(judge_server::route)
            .configure(status::route)
            .configure(region::route)
            .configure(contest::route)
            .configure(test_case::route)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}