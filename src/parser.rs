use serde::{ Serialize};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use toml::Value as TomlValue;
use thiserror::Error;
use std::fs;
use std::env;
use regex::Regex;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("unsupported file format")]
    Unsupported,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DynamicConfig {
    Yaml(YamlValue),
    Toml(TomlValue),
    Json(JsonValue),
}
// di parser.rs
impl DynamicConfig {
    pub fn get(&self, path: &str) -> Option<serde_json::Value> {
        let keys: Vec<&str> = path.split('.').collect();
        match self {
            DynamicConfig::Json(val) => get_nested_json_owned(val, &keys),
            DynamicConfig::Yaml(val) => {
                let json_val: serde_json::Value = serde_yaml::from_value(val.clone()).ok()?;
                get_nested_json_owned(&json_val, &keys)
            }
            DynamicConfig::Toml(val) => {
                let json_val: serde_json::Value = toml_to_json(val.clone());
                get_nested_json_owned(&json_val, &keys)
            }
        }
    }
}

fn get_nested_json_owned(val: &serde_json::Value, keys: &[&str]) -> Option<serde_json::Value> {
    if keys.is_empty() {
        return Some(val.clone());
    }
    let first = keys[0];
    val.get(first).and_then(|v| get_nested_json_owned(v, &keys[1..]))
}


/// Rekursif ambil value JsonValue dari keys
fn get_nested_json<'a>(val: &'a JsonValue, keys: &[&str]) -> Option<&'a JsonValue> {
    if keys.is_empty() {
        return Some(val);
    }
    let first = keys[0];
    val.get(first).and_then(|v| get_nested_json(v, &keys[1..]))
}

/// Konversi TomlValue ke JsonValue
fn toml_to_json(val: TomlValue) -> JsonValue {
    serde_json::to_value(val).unwrap_or(JsonValue::Null)
}

/// Expand environment variable ${VAR}
fn expand_env_vars(content: &str) -> String {
    let re = Regex::new(r"\$\{([A-Za-z0-9_]+)\}").unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let var_name = &caps[1];
        env::var(var_name).unwrap_or_else(|_| caps[0].to_string())
    }).to_string()
}

/// Load config file dinamis (YAML / TOML / JSON)
pub fn load_config(path: &str) -> Result<DynamicConfig, ParseError> {
    let content = fs::read_to_string(path)?;
    let expanded = expand_env_vars(&content);

    if path.ends_with(".yaml") || path.ends_with(".yml") {
        Ok(DynamicConfig::Yaml(serde_yaml::from_str(&expanded)?))
    } else if path.ends_with(".toml") {
        Ok(DynamicConfig::Toml(toml::from_str(&expanded)?))
    } else if path.ends_with(".json") {
        Ok(DynamicConfig::Json(serde_json::from_str(&expanded)?))
    } else {
        Err(ParseError::Unsupported)
    }
}
