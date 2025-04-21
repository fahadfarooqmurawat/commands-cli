use crate::{
    constants::{FOLDER_NAME, TOKEN_FILE},
    services::file_io::{read_text_from_file, save_text_to_file},
};

pub fn get_token() -> Result<String, String> {
    let token =
        read_text_from_file(FOLDER_NAME, TOKEN_FILE).map_err(|_e| format!("User not logged in"))?;

    return Ok(token);
}

pub fn save_token(token: &str) -> Result<(), String> {
    save_text_to_file(token, FOLDER_NAME, TOKEN_FILE)
        .map_err(|e| format!("Failed to save token: {}", e))?;

    return Ok(());
}
