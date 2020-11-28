use actix_web::{HttpResponse};
use juniper::http::graphiql::graphiql_source;
use crate::config::read_config;

pub async fn index() -> HttpResponse {
    let config = read_config();
    let gql_source = format!("http://{}:{}/graphql",
            config.server.address, config.server.port);
    let html = graphiql_source(gql_source.as_str());
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
