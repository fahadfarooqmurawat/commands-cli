use clap::Parser;
use command_cli::command::get_commands;
use command_cli::opts::Opts;
use command_cli::session::get_session_token;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let query = opts.args.join(" ");

    let session_token = get_session_token().await?;
    let commands = get_commands(&query, &session_token).await?;

    for command in commands {
        println!("{}", command);
    }
    Ok(())
}
