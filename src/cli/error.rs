use crate::{config::error::ConfigError, ha::error::HaError};
use inquire::error::InquireError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("inquire error: {0}")]
    Inquire(#[from] InquireError),

    #[error("Failed to save configuration.")]
    ConfigSave {
        #[source]
        source: ConfigError,
    },

    #[error("Failed to save secret.")]
    SecretSave {
        #[source]
        source: ConfigError,
    },

    #[error("Failed to delete config/secret.")]
    ConfigDeleteAll {
        #[source]
        source: ConfigError,
    },

    #[error("Authentication failed.")]
    AuthFailed,

    #[error("API error: {0}")]
    HaError(#[from] HaError),
}
