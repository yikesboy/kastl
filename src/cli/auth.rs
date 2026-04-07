use crate::cli::AuthCommands;
use crate::cli::error::AppError;
use crate::ha::HaClient;
use crate::ha::error::HaError;
use crate::config::{Config, ConfigProvider, Secret, SecretIdentifier};

use inquire::{Password, Text};

pub async fn handle(command: AuthCommands, config_manager: &impl ConfigProvider) -> Result<(), AppError> {
    match command {
        AuthCommands::Login => login(config_manager).await,
        AuthCommands::Logout => logout(),
    }
}

async fn login(config_manager: &impl ConfigProvider) -> Result<(), AppError> {
    let internal_url = Text::new("Home Assistant URL:").prompt()?;
    let bearer_token = Text::new("Long lived token:").prompt()?;

    let ha = HaClient::new(internal_url.clone(), bearer_token.clone());
    ha.api_status().await.map_err(|e| match e {
        HaError::Unauthorized => AppError::AuthFailed,
        e => AppError::HaError(e),
    })?;

    let config = Config {
        internal_url,
    };

    let secret = Secret {
        identifier: SecretIdentifier::BearerToken,
        value: bearer_token,
    };

    config_manager.save_config(&config).map_err(|e| AppError::ConfigSave { source: e })?;
    config_manager.save_secret(&secret).await.map_err(|e| AppError::SecretSave { source: e })?;

    println!("Logged In successfully.");
    Ok(())
}

fn logout() -> Result<(), AppError> {
    Ok(())
}
