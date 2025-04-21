use crate::{
    helpers::{get_token::get_token, get_user::get_user},
    services::api::post_command::post_command,
    utils::read_from_terminal::read_text_from_terminal,
};

pub async fn handle_add_command() -> Result<(), String> {
    let user = get_user().map_err(|_e| "User not logged in")?;
    let token = get_token().map_err(|_e| "User not logged in")?;

    let command = read_text_from_terminal("Command: ");
    let description = read_text_from_terminal("Description: ");

    let add_response = post_command(&user, token, command, description).await?;

    if !add_response.success {
        return Err("Failed to add command".into());
    }

    println!("Command added");

    // TODO: fetch alll commands and save them in the local database

    Ok(())
}
