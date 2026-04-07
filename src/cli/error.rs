use crate::{config::error::ConfigError, ha::error::HaError};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("inquire error: {0}")]
    Inquire(#[from] inquire::error::InquireError),

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

    #[error("Authentication failed.")]
    AuthFailed,

    #[error("API error: {0}")]
    HaError(#[from] HaError),
}
