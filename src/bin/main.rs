use clap::Parser;
use command_cli::{command::Command, opts::Opts};
use reqwest::{Client, Error, Response};
use serde::Deserialize;
use serde_json::json;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = Opts::parse();
    let query = opts.args.join(" ");

    let url = format!(
        "https://command.app.murawat.com/api/commands?search={}",
        query
    );

    let token_name = "__Secure-next-auth.session-token";
    let token_value = 	"eyJhbGciOiJkaXIiLCJlbmMiOiJBMjU2R0NNIn0..Q9r8jUP03Um9Knkj._PCds7GY_tf4Qd4k3jt8NIbQuzGf18_CefcW4RQ0qVvXAPDgxHXuEvAx2_co1vPGJcN9NdYTILHp2FIl-ExLVglWpyPAW0rvOV_D7E0FIabnToGOjwE_YXjuvaxt_KWwFJtjw8sHp6ZeK7yxxGFomsTYv1mCC42iDR4ZSkOYGPInimthJ4S7ClHTWTgPp3KIhmRLaV-T8K0aYHgSFZvJPcKcaq7OOkZitTJs6WGyEWvf3a687sLd_TMbkILPLThWzXYEFF99XYZd7sr-GNVSoPZ5Z0GkdYdA_Em0NAs3g5ClP0FA33uV_2NnF6Kcj2OL-NBRKO1ubBu0FaUVBN36WKUd9mZWUwo_BmIKbBl0Xcuv8ifbQW3A-IjivtZCvGdS31VrCVA.dhncWLL5uSoKrbFCahPh9Q";

    let client = Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("{}={}", token_name, token_value))
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Vec<Command>>().await {
                    Ok(commands) => {
                        for command in commands {
                            println!(
                                "\nCommand: {}\nDescription: {}\n",
                                command.command, command.description
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize response: {}", e);
                        exit(1);
                    }
                }
            } else {
                eprintln!("Request failed with status: {}", resp.status());
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("Request to server failed: {}", e);
            exit(1);
        }
    }

    return Ok(());
}
