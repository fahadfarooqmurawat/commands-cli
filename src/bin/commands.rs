use clap::Parser;
use command_cli::cli_opts::{CliCommand, CliOpts};
use command_cli::commands_handlers::handle_add::handle_add;
use command_cli::commands_handlers::handle_login::handle_login;
use command_cli::commands_handlers::handle_logout::handle_logout;
use command_cli::commands_handlers::handle_search::handle_search;
use command_cli::commands_handlers::handle_update::handle_update;
use command_cli::commands_handlers::handle_user::handle_user;
use command_cli::helpers::version_checker::version_checker;
use command_cli::utils::write_in_color::{write_in_red, write_in_yellow};

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
        CliCommand::Search { query } => handle_search(query).await,
        CliCommand::Add => handle_add().await,
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
