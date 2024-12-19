use core::fmt;

use reqwest::Client;
use serde::Deserialize;

const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const WHITE: &str = "\x1b[0m";
const TOKEN_NAME: &str = "__Secure-next-auth.session-token";
const BASE_URL: &str = "https://command.app.murawat.com/api/commands";

#[derive(Deserialize, Debug)]
pub struct Command {
    pub command: String,
    pub description: String,
}
impl fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}\n{}{}{}\n",
            GREEN, self.command, BLUE, self.description, WHITE
        )
    }
}

pub async fn get_commands(query: &String, token_value: &str) -> Result<Vec<Command>, String> {
    let url = format!("{}?search={}", BASE_URL, query);
    let client = Client::new();

    let response = client
        .get(url)
        .header("Cookie", format!("{}={}", TOKEN_NAME, token_value))
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let commands_response = resp.json::<Vec<Command>>().await;

                match commands_response {
                    Ok(commands) => {
                        return Ok(commands);
                    }
                    Err(_e) => {
                        return Err("Failed to retrieve commands".to_string());
                    }
                }
            } else {
                return Err("Failed to retrieve commands".to_string());
            }
        }
        Err(_e) => {
            return Err("Failed to retrieve commands".to_string());
        }
    }
}
