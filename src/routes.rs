use crate::gqlweb::{index, graphql};
use actix_web::web;

pub fn core(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
    cfg.route("/graphql", web::post().to(graphql::graphql));
}
