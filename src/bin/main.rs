use clap::Parser;

use command_cli::cli_command::CliCommand;
use command_cli::cli_opts::CliOpts;
use command_cli::command::{get_commands, Command};
use command_cli::session::get_session_token;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_opts = CliOpts::parse();
    let cli_command: CliCommand = cli_opts.try_into().unwrap();

    println!("args {:?} ", cli_command);
    // let query = opts.args.join(" ");

    // let session_token = get_session_token().await?;
    // let commands = get_commands(&query, &session_token).await?;

    // for command in commands {
    // println!("{}", command);
    // }
    Ok(())
}
