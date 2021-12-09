use std::env;
use std::fs::File;
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub hostname: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RepoConfig {
    pub owner: String,
    pub name: String,
    pub branch: String,
    pub secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientConfig {
    pub user: String,
    pub hostname: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub repo: RepoConfig,
    pub client: ClientConfig,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load the "default" config
        let base_dir = env::current_dir()?;
        let file_path = base_dir.join("config/zinc.json");

        let file = File::open(&file_path)?;

        let config: Config = serde_json::from_reader(file)?;
        Ok(config)
    }

    pub fn bind_address(&self) -> SocketAddr {
        let addr = format!("{}:{}", &self.server.hostname, &self.server.port);
        addr.parse()
            .expect("failed to parse server hostname and port into valid socket address")
    }
}
