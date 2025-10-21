use crate::AppConfig;

#[derive(Debug)]
pub enum ValidationError {
    MissingField(&'static str),
    InvalidPort(u16),
}

pub fn validate_config(cfg: &AppConfig) -> Result<(), ValidationError> {
    if cfg.database.url.is_empty() {
        return Err(ValidationError::MissingField("database.url"));
    }

    if !(1..=65535).contains(&cfg.server.port) {
        return Err(ValidationError::InvalidPort(cfg.server.port));
    }

    Ok(())
}
