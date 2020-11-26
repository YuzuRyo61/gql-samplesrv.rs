use std::{fs, process};
use std::io::{BufReader, Read};

#[derive(Debug, Deserialize)]
pub struct Config {
    server: Server
}

#[derive(Debug, Deserialize)]
pub struct Server {
    address: String,
    port: u16
}

fn read_file() -> Result<String, String> {
    let mut file_content = String::new();
    let mut fr = fs::File::open("gql_config.toml")
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;
    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

pub fn read_config() -> Config {
    let cfg_file = match read_file() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Can't open gql_config.toml: {}", e);
            process::exit(1);
        }
    };

    match toml::from_str(&cfg_file) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed parse gql_config.toml: {}", e);
            process::exit(2);
        }
    }
}
