use crate::cli::AuthCommands;
use crate::cli::error::AppError;
use crate::cli::util::with_spinner;
use crate::config::{Config, ConfigProvider, Secret, SecretIdentifier};
use crate::ha::HaClient;
use crate::ha::error::HaError;

use inquire::{Confirm, Text};

pub async fn handle(
    command: AuthCommands,
    config_manager: &impl ConfigProvider,
) -> Result<(), AppError> {
    match command {
        AuthCommands::Login => login(config_manager).await,
        AuthCommands::Logout => logout(config_manager).await,
    }
}

async fn login(config_manager: &impl ConfigProvider) -> Result<(), AppError> {
    let internal_url = Text::new("Home Assistant URL:").prompt()?;
    let bearer_token = Text::new("Long lived token:").prompt()?;

    let ha = HaClient::new(internal_url.clone(), bearer_token.clone());

    with_spinner("Testing connection and token...", ha.api_status())
        .await
        .map_err(|e| match e {
            HaError::Unauthorized => AppError::AuthFailed,
            e => AppError::HaError(e),
        })?;

    let config = Config { internal_url };

    let secret = Secret {
        identifier: SecretIdentifier::BearerToken,
        value: bearer_token,
    };

    config_manager
        .save_config(&config)
        .map_err(|e| AppError::ConfigSave { source: e })?;
    config_manager
        .save_secret(&secret)
        .await
        .map_err(|e| AppError::SecretSave { source: e })?;

    println!("Logged In successfully.");
    Ok(())
}

async fn logout(config_manager: &impl ConfigProvider) -> Result<(), AppError> {
    with_spinner(
        "Deleting config and secrets...",
        config_manager.delete_all(),
    )
    .await
    .map_err(|e| AppError::ConfigDeleteAll { source: e })?;

    let answer = Confirm::new("Do you want to login again?").prompt()?;
    match answer {
        true => login(config_manager).await,
        false => Ok(()),
    }
}
