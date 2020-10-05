use std::io;

use actix_web::client::Client;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};

use online_judge::graphql_schema::*;

mod encryption;
use encryption::encode;

async fn handle_heartbeat() -> impl Responder {
    println!("recieved judge_server heartbeat");
    HttpResponse::Ok()
}

async fn ping_judge_server() -> impl Responder {
    let token = encode::sha256_token("YOUR_TOKEN_HERE");
   
    let response = Client::new()
        .post("http://127.0.0.1:12358/ping")
        .set_header("X-Judge-Server-Token", token)
        .set_header("Content-Type", "application/json")
        .send()
        .await;
    println!("Response: {:?}", response);
    HttpResponse::Ok().body("ping sended")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let docker_port = "172.17.0.1:8080";
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(schema::create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql_config)
            .service(web::resource("/judge_server/heartbeat").route(web::post().to(handle_heartbeat)))
            .service(web::resource("/api/ping_judge_server").route(web::post().to(ping_judge_server)))
    })
    .bind("127.0.0.1:8080")?
    .bind(docker_port)?
    .run()
    .await
}