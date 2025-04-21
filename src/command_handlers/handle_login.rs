use crate::{
    constants::{FOLDER_NAME, TOKEN_FILE, USER_FILE},
    services::{api::post_login::post_login, file_io::save_text_to_file},
    utils::read_from_terminal::{read_password_from_terminal, read_text_from_terminal},
};

pub async fn handle_login() -> Result<(), String> {
    let email = read_text_from_terminal("Email: ");
    let password = read_password_from_terminal("Password: ");

    let login_response = post_login(email, password).await?;
    let token = login_response.get_token();
    let user = login_response.get_user();
    let user_str = serde_json::to_string_pretty(user)
        .map_err(|e| format!("Failed to parse user data: {}", e))?;

    save_text_to_file(token, FOLDER_NAME, TOKEN_FILE)
        .map_err(|e| format!("Failed to save token: {}", e))?;
    save_text_to_file(&user_str, FOLDER_NAME, USER_FILE)
        .map_err(|e| format!("Failed to save user: {}", e))?;

    println!("Welcome {}", user.get_name());

    Ok(())
}
