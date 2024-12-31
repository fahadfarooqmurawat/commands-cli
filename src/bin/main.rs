use clap::Parser;
use command_cli::auth::{login, logout};
use command_cli::cli_opts::{CliCommand, CliOpts};
use command_cli::file_io::read_user;
use command_cli::search::search;
use command_cli::version_checker::{check_latest_version, update_to_latest};
use command_cli::write_in_color::write_in_color;
use termcolor::Color;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_options = CliOpts::parse();

    if cli_options.command != CliCommand::Update {
        if let Err(_e) = check_latest_version().await {
            return Ok(());
        }
    }

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
                let _ = write_in_color("Not logged in".into(), Color::Red);
            }
        }
        CliCommand::Search { query } => {
            let search_text = query.join(" ");

            if let Err(e) = search(search_text).await {
                println!("{}", e);
            }
        }
        CliCommand::Update {} => {
            if let Err(e) = update_to_latest().await {
                println!("{}", e);
            }
        }
    }

    Ok(())
}
