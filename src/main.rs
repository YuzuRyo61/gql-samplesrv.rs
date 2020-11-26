#[macro_use]
extern crate serde_derive;
extern crate toml;

use actix_web::{HttpServer, App};

mod config;
mod http;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg: config::Config = config::read_config();
    let bind : String = format!("{}:{}", cfg.server.address, cfg.server.port);

    HttpServer::new(|| App::new()
        .service(http::index))
        .bind(bind)?
        .run()
        .await
}
