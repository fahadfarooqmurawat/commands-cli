use crate::{
    helpers::{sync_commands, token::get_token, user::get_user},
    services::api::get_commands::get_commands,
};

pub async fn handle_sync() -> Result<(), String> {
    let user = get_user()?;
    let token = get_token()?;

    let commands = get_commands(&user, token).await?;

    sync_commands(&commands).await?;
    // TODO: sync db with the server
    println!("Synced");

    Ok(())
}
