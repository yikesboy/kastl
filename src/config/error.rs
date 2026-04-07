#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("could not determine config directory")]
    NoConfigDir,

    #[error("could not convert config struct to toml: {0}")]
    UnableToConvertToToml(#[from] toml::ser::Error),

    #[error("could not convert toml to config struct: {0}")]
    UnableToConvertFromToml(#[from] toml::de::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("keyring error: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("background task failed: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),
}
