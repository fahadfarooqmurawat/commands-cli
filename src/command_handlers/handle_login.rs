use crate::{
    helpers::{token::save_token, user::save_user},
    services::api::post_login::post_login,
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

    save_token(token)?;
    save_user(&user_str)?;

    println!("Welcome {}", user.get_name());

    Ok(())
}
