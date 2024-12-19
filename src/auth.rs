use dirs_next;
use reqwest::Client;
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct User {
    user_id: u32,
    user_name: String,
    user_email: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
    user: User,
}

fn get_email() -> String {
    println!("Email: ");
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    email.trim().to_string()
}

fn get_password() -> String {
    println!("Password: ");
    read_password().unwrap()
}

async fn make_login_request(email: String, password: String) -> Result<LoginResponse, String> {
    let client = Client::new();

    let response = client
        .post("http://localhost:5003/login")
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if response.status().is_success() {
        response
            .json::<LoginResponse>()
            .await
            .map_err(|e| format!("Failed to parse response JSON: {}", e))
    } else {
        Err(format!("Login failed: {}", response.status()))
    }
}

fn save_token(token: &str) -> std::io::Result<()> {
    let mut config_dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push("my_cli");

    fs::create_dir_all(&config_dir)?;

    let token_path = config_dir.join("token.txt");
    fs::write(token_path, token)
}

fn save_user(user: &User) -> std::io::Result<()> {
    let mut config_dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push("my_cli");

    fs::create_dir_all(&config_dir)?;

    let user_path = config_dir.join("user.json");
    let user_data = serde_json::to_string_pretty(user).expect("Failed to serialize user");
    fs::write(user_path, user_data)
}

pub async fn login() {
    let email = get_email();
    let password = get_password();

    match make_login_request(email, password).await {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        Ok(login_response) => {
            let token = login_response.token;
            let user = login_response.user;

            if let Err(e) = save_token(&token) {
                eprintln!("Failed to save token: {}", e);
                std::process::exit(1);
            };

            if let Err(e) = save_user(&user) {
                eprintln!("Failed to save user: {}", e);
                std::process::exit(1);
            };

            println!("Login successful");
            println!("Welcome {}", user.user_name);
        }
    }
}
