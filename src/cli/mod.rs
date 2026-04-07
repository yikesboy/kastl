pub mod auth;
pub mod error;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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
    Entities {
        #[command(subcommand)]
        command: EntityCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum AuthCommands {
    Login,
    Logout,
}

#[derive(Subcommand, Debug)]
pub enum EntityCommands {
    List {
        #[arg(long)]
        domain: Option<String>,
    },
    Get {
        entity_id: String,
    },
    Set {
        entity_id: String,
        state: String,
    },
    // Delete {
    //     entity_id: String,
    // },
}
