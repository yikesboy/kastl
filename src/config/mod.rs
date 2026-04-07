pub mod error;
pub mod secret;
pub mod storage;

use error::ConfigError;
use secret::SecretStorage;
use serde::{Deserialize, Serialize};
use storage::ConfigStorage;
use strum;

pub(crate) const SERVICE_IDENTIFIER: &str = "kastl-cli";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub internal_url: String,
}

#[derive(Debug, Clone, strum::Display, strum::EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum SecretIdentifier {
    BearerToken,
}

#[derive(Debug, Clone)]
pub struct Secret {
    pub identifier: SecretIdentifier,
    pub value: String,
}

pub trait ConfigProvider {
    fn save_config(&self, config: &Config) -> Result<(), ConfigError>;
    fn load_config(&self) -> Result<Config, ConfigError>;
    async fn save_secret(&self, secret: &Secret) -> Result<(), ConfigError>;
    async fn load_secret(&self, secret_identifier: SecretIdentifier) -> Result<Secret, ConfigError>;
    async fn delete_all(&self) -> Result<(), ConfigError>;
}

pub struct ConfigService<S: SecretStorage, C: ConfigStorage> {
    secret_storage: S,
    config_storage: C,
}

impl<S: SecretStorage, C: ConfigStorage> ConfigService<S, C> {
    pub fn new(secret_storage: S, config_storage: C) -> Self {
        Self {
            secret_storage,
            config_storage,
        }
    }
}

impl<S: SecretStorage, C: ConfigStorage> ConfigProvider for ConfigService<S, C> {
    fn save_config(&self, config: &Config) -> Result<(), ConfigError> {
        self.config_storage.save(&config)
    }

    fn load_config(&self) -> Result<Config, ConfigError> {
        self.config_storage.load()
    }

    async fn save_secret(&self, secret: &Secret) -> Result<(), ConfigError> {
        self.secret_storage.save(secret).await
    }

    async fn load_secret(&self, secret_identifier: SecretIdentifier) -> Result<Secret, ConfigError> {
        self.secret_storage.load(secret_identifier).await
    }

    async fn delete_all(&self) -> Result<(), ConfigError> {
        self.secret_storage.delete_all().await?;
        self.config_storage.delete()?;
        Ok(())
    }
}
