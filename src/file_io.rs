use crate::user::User;
use dirs_next;
use std::fs;
use std::path::PathBuf;

pub fn save_token(token: &str) -> std::io::Result<()> {
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

    fs::read_to_string(token_path).ok()
}

pub fn delete_token() -> std::io::Result<()> {
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

pub fn save_user(user: &User) -> std::io::Result<()> {
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

    fs::read_to_string(user_path)
        .ok()
        .and_then(|data| serde_json::from_str::<User>(&data).ok())
}

pub fn delete_user() -> std::io::Result<()> {
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
