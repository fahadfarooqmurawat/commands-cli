use clap::Parser;

use command_cli::auth::{login, logout, read_user};
use command_cli::cli_command::{self, CliCommand};
use command_cli::cli_opts::CliOpts;
use command_cli::command::{get_commands, Command};
use command_cli::session::get_session_token;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_opts = CliOpts::parse();
    let cli_command: CliCommand = cli_opts.try_into().unwrap();

    println!("Cli Command: {:?} ", cli_command);

    match cli_command {
        CliCommand::Info => {
            let result = read_user();

            if let Some(user) = result {
                println!("{}", user.get_name());
                println!("{}", user.get_email());
            } else {
                println!("Not logged in");
            }
        }
        CliCommand::Login => {
            login().await;
        }
        CliCommand::Logout => {
            logout().unwrap();
        }
        CliCommand::Search(_) => todo!(),
    }

    // let query = opts.args.join(" ");

    // let session_token = get_session_token().await?;
    // let commands = get_commands(&query, &session_token).await?;

    // for command in commands {
    // println!("{}", command);
    // }
    Ok(())
}
