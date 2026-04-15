mod cli;
mod ha;
mod config;

use clap::Parser;
use cli::Cli;

use crate::{cli::error::AppError, config::{ConfigService, secret::KeyringSecretStorage, storage::{ConfigStorage, FsConfigStorage}}};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let cli = Cli::parse();
    println!("{:?}", cli.command);

    let config_manager = ConfigService::new(KeyringSecretStorage, FsConfigStorage);

    match cli.command {
        Some(command) => {
            match command {
                cli::Commands::Auth { command } => cli::auth::handle(command, &config_manager).await?,
                cli::Commands::Entities { command } => unreachable!(),
            }
        },
        None => unreachable!(),
    }

    Ok(())
}
