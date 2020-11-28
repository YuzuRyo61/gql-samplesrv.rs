extern crate gql_samplesrv;

use actix_web::{HttpServer, App, middleware};
use gql_samplesrv::{config, routes};
use gql_samplesrv::gql_schema::create_schema;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg: config::Config = config::read_config();

    let schema = std::sync::Arc::new(create_schema());

    let addr = format!("{}:{}", cfg.server.address, cfg.server.port);

    HttpServer::new(move || {
      App::new()
          .data(schema.clone())
          .wrap(middleware::Logger::default())
          .wrap(
              Cors::default()
                  .allow_any_origin()
                  .allowed_methods(vec!["GET", "POST"])
                  .max_age(3600)
          )
          .configure(routes::core)
    })
        .bind(addr)?
        .run()
        .await
}
