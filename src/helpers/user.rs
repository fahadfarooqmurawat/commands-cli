use crate::{
    constants::{FOLDER_NAME, USER_FILE},
    objects::user::User,
    services::file_io::{read_text_from_file, save_text_to_file},
};

pub fn get_user() -> Result<User, String> {
    let user_str =
        read_text_from_file(FOLDER_NAME, USER_FILE).map_err(|_e| format!("User not logged in"))?;

    let user =
        serde_json::from_str::<User>(&user_str).map_err(|_e| format!("User not logged in"))?;

    return Ok(user);
}

pub fn save_user(user_str: &str) -> Result<(), String> {
    save_text_to_file(&user_str, FOLDER_NAME, USER_FILE)
        .map_err(|e| format!("Failed to save user: {}", e))?;

    return Ok(());
}
