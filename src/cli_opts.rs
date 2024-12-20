use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliOpts {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    Login,
    Logout,
    User,
    #[command(about = "Searches for the key phrase in the database")]
    Search {
        query: Vec<String>,
    },
}
