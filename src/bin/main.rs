use clap::Parser;

use command_cli::auth::{login, logout};
use command_cli::cli_opts::{CliCommand, CliOpts};
use command_cli::file_io::read_user;
use command_cli::search::search;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_options = CliOpts::parse();

    match cli_options.command {
        CliCommand::Login => {
            if let Err(e) = login().await {
                println!("{}", e);
            }
        }
        CliCommand::Logout => {
            if let Err(e) = logout() {
                println!("{}", e);
            }
        }
        CliCommand::User => {
            if let Some(user) = read_user() {
                println!("{}", user);
            } else {
                println!("Not logged in");
            }
        }
        CliCommand::Search { query } => {
            let search_text = query.join(" ");

            if let Err(e) = search(search_text).await {
                println!("{}", e);
            }
        }
    }

    Ok(())
}
