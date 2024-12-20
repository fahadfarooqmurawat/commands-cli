use clap::Parser;

use clipboard::{ClipboardContext, ClipboardProvider};
use command_cli::api::make_search_request;
use command_cli::auth::{login, logout};
use command_cli::cli_command::CliCommand;
use command_cli::cli_opts::CliOpts;
use command_cli::file_io::{read_token, read_user};
use command_cli::user_input::get_number;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_opts = CliOpts::parse();
    let cli_command: CliCommand = cli_opts.try_into().unwrap_or_else(|e| {
        eprintln!("Failed to parse CLI options: {}", e);
        std::process::exit(1);
    });

    println!("Cli Command: {:?} ", cli_command);

    match cli_command {
        CliCommand::Info => {
            if let Some(user) = read_user() {
                println!("{}", user.get_name());
                println!("{}", user.get_email());
            } else {
                println!("Not logged in");
            }
        }
        CliCommand::Login => {
            if let Err(e) = login().await {
                println!("{}", e);
            }
        }
        CliCommand::Logout => {
            if let Err(e) = logout() {
                println!("{}", e);
            } else {
                println!("Logged out.")
            }
        }
        CliCommand::Search(search_text) => {
            let user = match read_user() {
                Some(user) => user,
                None => {
                    println!("Not logged in");
                    return Ok(());
                }
            };

            let token = match read_token() {
                Some(token) => token,
                None => {
                    println!("Not loggd in");
                    return Ok(());
                }
            };

            match make_search_request(&user, token, search_text).await {
                Ok(commands) => {
                    if commands.is_empty() {
                        println!("No commands found");
                        return Ok(());
                    }

                    if commands.len() == 1 {
                        println!("{}", commands[0].get_command());
                        copy_to_clipboard(commands[0].get_command().into());
                        return Ok(());
                    }

                    for (index, command) in commands.iter().enumerate() {
                        println!("{}: {}", index, command);
                    }

                    let command_index = match get_number() {
                        Ok(num) => num,
                        Err(e) => {
                            println!("{}", e);
                            return Ok(());
                        }
                    };

                    let selected_command = &commands[command_index];

                    copy_to_clipboard(selected_command.get_command().into());
                }
                Err(e) => {
                    println!("Search failed: {}", e);
                }
            }
        }
    }

    Ok(())
}

fn copy_to_clipboard(text: String) -> () {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    if let Err(_e) = ctx.set_contents(text) {
        println!("Failed to copy to clipboard");
    } else {
        println!("Copied to clipboard");
    }
}
