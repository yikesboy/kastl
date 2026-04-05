use crate::config::error::ConfigError;
use crate::config::{Config, SERVICE_IDENTIFIER};
use std::{
    fs::{create_dir_all, read_to_string, remove_file, write},
    path::PathBuf,
};

const CONFIG_FILE_NAME: &str = "config.toml";

pub trait ConfigStorage {
    fn save(&self, config: &Config) -> Result<(), ConfigError>;
    fn load(&self) -> Result<Config, ConfigError>;
    fn delete(&self) -> Result<(), ConfigError>;
    fn app_config_dir(&self) -> Result<PathBuf, ConfigError>;
    fn app_config_file(&self) -> Result<PathBuf, ConfigError>;
}

pub struct FsConfigStorage;

impl ConfigStorage for FsConfigStorage {
    fn save(&self, config: &Config) -> Result<(), ConfigError> {
        let config_file = self.app_config_file()?;
        let file_content: String = toml::to_string(&config)?;
        write(&config_file, &file_content)?;
        Ok(())
    }

    fn load(&self) -> Result<Config, ConfigError> {
        let config_file = self.app_config_file()?;
        let file_content = read_to_string(config_file)?;
        Ok(toml::from_str(&file_content)?)
    }

    fn delete(&self) -> Result<(), ConfigError> {
        let config_file = self.app_config_file()?;
        remove_file(config_file)?;
        let app_config_dir = self.app_config_dir()?;
        std::fs::remove_dir(app_config_dir)?;
        Ok(())
    }

    fn app_config_dir(&self) -> Result<PathBuf, ConfigError> {
        let config_path = dirs::config_dir()
            .ok_or(ConfigError::NoConfigDir)?
            .join(SERVICE_IDENTIFIER);
        create_dir_all(&config_path)?;
        Ok(config_path)
    }

    fn app_config_file(&self) -> Result<PathBuf, ConfigError> {
        Ok(self.app_config_dir()?.join(CONFIG_FILE_NAME))
    }
}
