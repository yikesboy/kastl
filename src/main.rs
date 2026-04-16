mod cli;
mod config;
mod ha;

use clap::Parser;
use cli::Cli;

use crate::{
    cli::error::AppError,
    config::{ConfigService, secret::KeyringSecretStorage, storage::FsConfigStorage},
    ha::HaClient,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let cli = Cli::parse();
    let config_manager = ConfigService::new(KeyringSecretStorage, FsConfigStorage);
    if let Some(cli::Commands::Auth { command }) = cli.command {
        return cli::auth::handle(command, &config_manager).await;
    }

    let ha = HaClient::from_config(&config_manager).await?;
    match cli.command {
        Some(command) => match command {
            cli::Commands::Auth { command: _ } => unreachable!(),
            cli::Commands::Config => cli::config::handle(&ha).await?,
        },
        None => unreachable!(),
    }

    Ok(())
}
