use crate::gqlweb::{index};
use actix_web::web;

pub fn core(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
}
