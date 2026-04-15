use crate::config::error::ConfigError;
use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum HaError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("decode error: {source}\nbody: {body}")]
    Decode {
        source: serde_json::Error,
        body: String,
    },

    #[error("HTTP error. Status: {status}, Body: {body}")]
    Http { status: StatusCode, body: String },

    #[error("Unauthorized.")]
    Unauthorized,

    #[error("config error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("entityid missing, need atleast one")]
    MissingEntityId,
}
