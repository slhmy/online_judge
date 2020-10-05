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

use actix_web::*;
use actix_session::*;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;
use schema::*;

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

pub fn graphql_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("").wrap(
            CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                .secure(false),
        )
            .service(web::resource("/graphql")
                .route(web::post().to(graphql)))
            .service(web::resource("/graphiql")
                .route(web::get().to(graphiql)))
    );
}
