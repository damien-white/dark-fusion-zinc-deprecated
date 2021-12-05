use std::env;
use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
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
pub struct ServerConfig {
    pub user: String,
    pub hostname: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub app: AppConfig,
    pub repo: RepoConfig,
    pub server: ServerConfig,
    pub scripts: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load the "default" config
        let base_dir = env::current_dir()?;
        let file_path = base_dir.join("config.yml");

        let file = File::open(&file_path)?;

        let config: Config = serde_yaml::from_reader(file)?;
        Ok(config)
    }
}
