#[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;

fn main() {
    let cfg: config::Config = config::read_config();
    println!("{:#?}", cfg);
}
