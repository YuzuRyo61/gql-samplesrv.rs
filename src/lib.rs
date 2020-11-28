#[macro_use]
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod config;
pub mod gqlweb;
pub mod routes;

pub mod schema;
pub mod models;

pub mod gql_schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}
