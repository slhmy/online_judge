mod schema;
mod service;
mod database;
mod user;
mod problem;
mod graphql;
mod judge_server;
mod utils;
mod encryption;
mod errors;

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
    collections::{ BTreeMap, HashMap },
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
    judge_server::model::JudgeServerInfo,
};
use dotenv::dotenv;
use std::env;

lazy_static! {
    static ref ACCESS_KEY_ID: String = {
        dotenv().ok();
        env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID must be set")
    };
    static ref ACCESS_SECRET: String = {
        dotenv().ok();
        env::var("ACCESS_SECRET").expect("ACCESS_SECRET must be set")
    };
    static ref DATABASE_URL: String = {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")  
    };
    static ref JUDGE_SERVER_INFOS: RwLock<HashMap<String, JudgeServerInfo>> = RwLock::new(HashMap::new());
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
                      .path("/")
                      .http_only(false)
                      .max_age(1800)    
                      .visit_deadline(Duration::minutes(30))
                      .secure(false)))
            .configure(graphql::route)
            .configure(user::route)
            .configure(problem::route)
            .configure(judge_server::route)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}