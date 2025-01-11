use crate::{
    helpers::{get_token::get_token, get_user::get_user},
    services::api::request_post_add,
    utils::read_from_terminal::{read_password_from_terminal, read_text_from_terminal},
};

pub async fn handle_add() -> Result<(), String> {
    let user = get_user().map_err(|_e| "User not logged in")?;
    let token = get_token().map_err(|_e| "User not logged in")?;

    let command = read_text_from_terminal("Command: ");
    let description = read_password_from_terminal("Description: ");

    let add_response = request_post_add(&user, token, command, description).await?;

    if !add_response.success {
        return Err("Failed to add command".into());
    }

    println!("Command added");

    Ok(())
}
