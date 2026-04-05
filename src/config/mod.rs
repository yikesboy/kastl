mod error;
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
    pub username: String,
}

#[derive(Debug, strum::Display, strum::EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum SecretIdentifier {
    Password,
}

#[derive(Debug)]
pub struct Secret {
    pub identifier: SecretIdentifier,
    pub value: String,
}

pub struct ConfigManager<S: SecretStorage, C: ConfigStorage> {
    secret_storage: S,
    config_storage: C,
}

impl<S: SecretStorage, C: ConfigStorage> ConfigManager<S, C> {
    pub fn new(secret_storage: S, config_storage: C) -> Self {
        Self {
            secret_storage,
            config_storage,
        }
    }

    pub fn save_config(&self, config: &Config) -> Result<(), ConfigError> {
        self.config_storage.save(&config)
    }

    pub fn load_config(&self) -> Result<Config, ConfigError> {
        self.config_storage.load()
    }

    pub fn save_secret(&self, secret: &Secret) -> Result<(), ConfigError> {
        self.secret_storage.save(secret)
    }

    pub fn load_secret(&self, secret_identifier: SecretIdentifier) -> Result<Secret, ConfigError> {
        self.secret_storage.load(secret_identifier)
    }

    pub fn delete_all(&self) -> Result<(), ConfigError> {
        self.secret_storage.delete_all()?;
        self.config_storage.delete()?;
        Ok(())
    }
}
