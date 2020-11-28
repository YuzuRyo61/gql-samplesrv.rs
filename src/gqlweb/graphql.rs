use actix_web::{web, HttpResponse, Error};
use std::sync::Arc;

use crate::gql_schema::{Schema};
use juniper::http::GraphQLRequest;

pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>
) -> Result<HttpResponse, Error> {
    let body = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    }).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}
