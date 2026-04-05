use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string, remove_file, write};

const SERVICE_IDENTIFIER: &str = "kastl-cli";
const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("could not determine config directory")]
    NoConfigDir,

    #[error("could not convert config struct to toml: {0}")]
    UnableToConvertToToml(toml::ser::Error),

    #[error("could not convert toml to config struct: {0}")]
    UnableToConvertFromToml(toml::de::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("keyring error: {0}")]
    Keyring(#[from] keyring::Error),
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub internal_url: String,
    pub username: String,
}

impl Config {
    pub fn new(internal_url: String, username: String) -> Self {
        Self {
            internal_url,
            username,
        }
    }

    pub fn save_config(&self) -> Result<(), ConfigError> {
        let file_content = toml::to_string(self).map_err(ConfigError::UnableToConvertToToml)?;

        let config_path = dirs::config_dir()
            .ok_or(ConfigError::NoConfigDir)?
            .join(SERVICE_IDENTIFIER);
        create_dir_all(&config_path)?;

        let config_file = config_path.join(CONFIG_FILE_NAME);
        write(&config_file, file_content)?;

        Ok(())
    }

    pub fn load_config() -> Result<Self, ConfigError> {
        let config_path = dirs::config_dir()
            .ok_or(ConfigError::NoConfigDir)?
            .join(SERVICE_IDENTIFIER);
        let config_file = config_path.join(CONFIG_FILE_NAME);
        let raw_file_content = read_to_string(config_file)?;
        toml::from_str(&raw_file_content).map_err(ConfigError::UnableToConvertFromToml)
    }

    pub fn save_password(&self, password: &str) -> Result<(), ConfigError> {
        Entry::new(SERVICE_IDENTIFIER, &self.username)?.set_password(password)?;
        Ok(())
    }

    pub fn load_password(&self) -> Result<String, ConfigError> {
        let password = Entry::new(SERVICE_IDENTIFIER, &self.username)?.get_password()?;
        Ok(password)
    }

    pub fn delete_all(&self) -> Result<(), ConfigError> {
        match Entry::new(SERVICE_IDENTIFIER, &self.username)?.delete_credential() {
            Ok(_) => {}
            Err(keyring::Error::NoEntry) => {}
            Err(e) => return Err(ConfigError::Keyring(e)),
        }

        let config_file = dirs::config_dir()
            .ok_or(ConfigError::NoConfigDir)?
            .join(SERVICE_IDENTIFIER)
            .join(CONFIG_FILE_NAME);

        match remove_file(config_file) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(ConfigError::Io(e)),
        }

        Ok(())
    }
}
