use crate::{
    constants::{FOLDER_NAME, UPDATED_FILE},
    services::file_io::{read_text_from_file, save_text_to_file},
};

pub fn get_last_updated() -> Result<String, String> {
    let last_updated = read_text_from_file(FOLDER_NAME, UPDATED_FILE)
        .map_err(|_e| format!("Last modified file not found"))?;

    return Ok(last_updated);
}

pub fn save_last_updated(last_updated: &str) -> Result<(), String> {
    save_text_to_file(last_updated, FOLDER_NAME, UPDATED_FILE)
        .map_err(|e| format!("Failed to save modified file: {}", e))?;

    return Ok(());
}
