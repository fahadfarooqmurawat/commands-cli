use clap::Parser;
use commands_cli::cli_opts::{CliCommand, CliOpts};
use commands_cli::command_handlers::handle_add_command::handle_add_command;
use commands_cli::command_handlers::handle_login::handle_login;
use commands_cli::command_handlers::handle_logout::handle_logout;
use commands_cli::command_handlers::handle_search_commands::handle_search_commands;
use commands_cli::command_handlers::handle_update::handle_update;
use commands_cli::command_handlers::handle_user::handle_user;
use commands_cli::helpers::version_checker::version_checker;
use commands_cli::utils::write_in_color::{write_in_red, write_in_yellow};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_options = CliOpts::parse();

    if cli_options.command != CliCommand::Update {
        match version_checker().await {
            Ok(None) => {}
            Ok(Some(msg)) => write_in_yellow(msg),
            Err(e) => {
                write_in_red(format!("{}\n", e));
                return Ok(());
            }
        }
    }

    let result = match cli_options.command {
        CliCommand::Search { query } => handle_search_commands(query).await,
        CliCommand::Add => handle_add_command().await,
        CliCommand::User => handle_user(),
        CliCommand::Login => handle_login().await,
        CliCommand::Logout => handle_logout(),
        CliCommand::Update => handle_update().await,
    };

    if let Err(e) = result {
        write_in_red(format!("{}\n", e));
    }

    return Ok(());
}
