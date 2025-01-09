use dirs_next;
use std::fs;
use std::path::PathBuf;

pub fn save_text_to_file(text: &str, folder: &str, file: &str) -> std::io::Result<()> {
    let mut config_dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push(folder);

    fs::create_dir_all(&config_dir)?;

    let token_path = config_dir.join(file);
    fs::write(token_path, text)
}

pub fn read_text_from_file(folder: &str, file: &str) -> Option<String> {
    let mut config_dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push(folder);

    let token_path = config_dir.join(file);

    fs::read_to_string(token_path).ok()
}

pub fn delete_file(folder: &str, file: &str) -> std::io::Result<()> {
    let mut config_dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push(folder);

    let token_path = config_dir.join(file);

    if token_path.exists() {
        fs::remove_file(token_path)?;
    }

    Ok(())
}
