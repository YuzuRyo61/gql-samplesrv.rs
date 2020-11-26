extern crate gql_samplesrv;

use actix_web::{HttpServer, App};
use gql_samplesrv::{config, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg: config::Config = config::read_config();

    HttpServer::new(|| {
      App::new()
          .configure(routes::core)
    })
        .bind(format!("{}:{}", cfg.server.address, cfg.server.port))?
        .run()
        .await
}
