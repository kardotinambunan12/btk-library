use crate::AppConfig;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("unsupported file format")]
    Unsupported,
}

pub fn load_config(path: &str) -> Result<AppConfig, ParseError> {
    let content = fs::read_to_string(path)?;
    if path.ends_with(".yaml") || path.ends_with(".yml") {
        Ok(serde_yaml::from_str(&content)?)
    } else if path.ends_with(".toml") {
        Ok(toml::from_str(&content)?)
    } else {
        Err(ParseError::Unsupported)
    }
}
