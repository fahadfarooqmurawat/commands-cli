use crate::{
    constants::{FOLDER_NAME, TOKEN_FILE, USER_FILE},
    services::{api::request_post_login, file_io::save_text_to_file},
    utils::read_from_terminal::{read_password_from_terminal, read_text_from_terminal},
};

pub async fn handle_login() -> Result<(), String> {
    let email = read_text_from_terminal("Email: ");
    let password = read_password_from_terminal("Password: ");

    let login_response = request_post_login(email, password).await?;
    let token = login_response.get_token();
    let user = login_response.get_user();
    let user_str = serde_json::to_string_pretty(user).unwrap();

    save_text_to_file(token, FOLDER_NAME, TOKEN_FILE).unwrap();
    save_text_to_file(&user_str, FOLDER_NAME, USER_FILE).unwrap();

    println!("Welcome {}", user.get_name());

    Ok(())
}
