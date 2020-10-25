mod schema;
mod service;
mod database;
mod user;
mod graphql;
mod judge_server;
mod utils;
mod encryption;

#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

extern crate regex;
extern crate dotenv;
extern crate env_logger;
extern crate serde_json;
extern crate pretty_env_logger;

use std::{
    io,
    sync::RwLock,
    collections::BTreeMap,
    time::SystemTime,
};
use regex::Regex;
use time::Duration;
use actix_web::{ 
    App,
    middleware, 
    HttpServer, 
};
use actix_identity::{
    CookieIdentityPolicy, 
    IdentityService,
};
use actix_cors::Cors;
use crate::{
    graphql::schema as graphql_schema,
    database::*,
};

lazy_static! {    
    static ref VERIFICATION_MAP: RwLock<BTreeMap<String, (String, SystemTime)>> = RwLock::new(BTreeMap::new());
    static ref RE_EMAIL: Regex = Regex::new(r"^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();
    static ref RE_MOBILE: Regex = Regex::new(r"^((13[0-9])|(14[5|7])|(15([0-3]|[5-9]))|(18[0,5-9]))\d{8}$").unwrap();
    static ref RE_PASSWORD: Regex = Regex::new(r"^\S{6,20}$").unwrap();
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info, actix_web=info");
    env_logger::init();

    // Create schema
    let addr = create_db_executor();

    // Create Juniper schema
    let graphql_schema = std::sync::Arc::new(graphql_schema::create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(State { db: addr.clone() })
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
                      .max_age(60)
                      .visit_deadline(Duration::minutes(30))
                      .secure(false)))
            .configure(graphql::route)
            .configure(user::route)
            .configure(judge_server::route)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}