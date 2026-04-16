pub mod auth;
pub mod config;
pub mod error;
pub mod util;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(subcommand_required = true, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
    Config,
}

#[derive(Subcommand, Debug)]
pub enum AuthCommands {
    Login,
    Logout,
}
