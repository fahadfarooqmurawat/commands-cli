use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliOpts {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    #[command(about = "Login to command app")]
    Login,
    #[command(about = "Logout from command app")]
    Logout,
    #[command(about = "Get logged in user information")]
    User,
    #[command(about = "Searches for the key phrase in the database")]
    Search { query: Vec<String> },
}
