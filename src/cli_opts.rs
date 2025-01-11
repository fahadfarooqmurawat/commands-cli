use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliOpts {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum CliCommand {
    #[command(
        visible_alias = "-s",
        about = "Search for the key phrase in the database"
    )]
    Search {
        #[arg(help = "Search query")]
        query: Vec<String>,
    },
    #[command(visible_alias = "-a", about = "Add new command")]
    Add,
    #[command(visible_alias = "-u", about = "Get logged in user's information")]
    User,
    #[command(about = "Login to command app")]
    Login,
    #[command(about = "Logout from command app")]
    Logout,
    #[command(about = "Update command cli")]
    Update,
}
