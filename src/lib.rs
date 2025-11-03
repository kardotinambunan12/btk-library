pub mod parser;
pub mod watcher;

use serde::{Deserialize, Serialize};
pub use parser::{DynamicConfig, load_config};
pub use watcher::ConfigWatcher;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}
