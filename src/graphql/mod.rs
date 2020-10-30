pub mod mutations;
pub mod queries;
pub mod objects;

pub mod schema {
    use juniper::RootNode;
    use super::mutations::*;
    use super::queries::*;
    
    pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;
    
    pub fn create_schema() -> Schema {
        Schema::new(QueryRoot {}, MutationRoot {})
    }
}

use juniper::Context as JuniperContext;
use crate::{
    database::*,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub db: web::Data<State>,
}

impl JuniperContext for Context {}

impl Context {
    pub fn new(db: web::Data<State>) -> Self {
        Self {
            db: db,
        }
    }
}

use actix_web::*;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use schema::*;

pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    db: web::Data<State>,
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context::new(db);
    let res = data.execute(&st, &ctx);
    let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

pub(super) fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").route(web::post().to(graphql)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql)));
}