use dirs_next;
use dotenv::dotenv;
use reqwest::Client;
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    user_id: u32,
    user_name: String,
    user_email: String,
}

impl User {
    pub fn get_name(&self) -> &str {
        return &self.user_name;
    }
    pub fn get_email(&self) -> &str {
        return &self.user_email;
    }
}
#[derive(Deserialize)]
struct LoginResponse {
    token: String,
    user: User,
}

fn get_app_key() -> String {
    dotenv().ok();
    env::var("APP_SECRET_KEY").expect("Key missing")
}

fn get_email() -> String {
    print!("Email: ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    email.trim().to_string()
}

fn get_password() -> String {
    print!("Password: ");
    io::stdout().flush().unwrap();
    read_password().unwrap()
}

async fn make_login_request(email: String, password: String) -> Result<LoginResponse, String> {
    let client = Client::new();

    let response = client
        .post("http://localhost:5004/login")
        .header("x-api-key", get_app_key())
        .json(&serde_json::json!({ "user_email": email, "user_password": password }))
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

pub fn read_token() -> Option<String> {
    let mut config_dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push("my_cli");

    let token_path = config_dir.join("token.txt");

    // Try to read the token from the file if it exists.
    fs::read_to_string(token_path).ok()
}

fn delete_token() -> std::io::Result<()> {
    let mut config_dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push("my_cli");

    let token_path = config_dir.join("token.txt");

    if token_path.exists() {
        fs::remove_file(token_path)?;
    } else {
        println!("No token found to delete.");
    }

    Ok(())
}

// Function to delete the user.json file (logout)

fn save_user(user: &User) -> std::io::Result<()> {
    let mut config_dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push("my_cli");

    fs::create_dir_all(&config_dir)?;

    let user_path = config_dir.join("user.json");
    let user_data = serde_json::to_string_pretty(user).expect("Failed to serialize user");
    fs::write(user_path, user_data)
}

pub fn read_user() -> Option<User> {
    let mut config_dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push("my_cli");

    let user_path = config_dir.join("user.json");

    // Try to read the user data from the file if it exists and deserialize it
    fs::read_to_string(user_path)
        .ok()
        .and_then(|data| serde_json::from_str::<User>(&data).ok())
}

fn delete_user() -> std::io::Result<()> {
    let mut config_dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push("my_cli");

    let user_path = config_dir.join("user.json");

    if user_path.exists() {
        fs::remove_file(user_path)?;
    } else {
        println!("No user data found to delete.");
    }

    Ok(())
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

pub fn logout() -> std::io::Result<()> {
    if let Err(e) = delete_token() {
        eprintln!("Error deleting token: {}", e);
        return Err(e);
    }

    if let Err(e) = delete_user() {
        eprintln!("Error deleting user data: {}", e);
        return Err(e);
    }

    println!("Logged out successfully.");
    Ok(())
}
