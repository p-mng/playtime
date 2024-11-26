use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add an app to the config file
    Add {
        /// Name of the application
        name: String,
        /// Path to the executable
        exe: String,
    },
    /// Remove an app from the config file
    Remove {
        /// Name of the app to remove
        name: String,
    },
    /// List all saved apps
    List,
    /// Start an app and record time to the config file
    #[clap(alias = "run")]
    Start {
        /// Name of the app to start
        name: String,
    },
    /// List all recorded sessions
    Sessions {
        /// Name of the app to print sessions for
        name: String,
    },
}
