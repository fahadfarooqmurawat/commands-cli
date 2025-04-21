use crate::{
    objects::user::User,
    services::{api::get_commands::get_commands, sqlite::sync_commands},
    utils::write_in_color::write_in_red,
};

use super::last_updated::get_last_updated;

pub async fn fetch_and_sync_commands(user: &User, token: String) -> Result<String, String> {
    let last_updated = get_last_updated().ok();

    match get_commands(user, token, last_updated).await {
        Ok(command_response) => {
            if let Some(commands) = command_response {
                if let Ok(()) = sync_commands(&commands) {
                    return Ok(format!(
                        "Your {} commands are now available offline as well",
                        commands.len()
                    ));
                } else {
                    return Ok("Failed to sync. Your commands are not available offline.".into());
                }
            } else {
                return Ok("Commands not updated since last fetch".into());
            }
        }
        Err(e) => {
            write_in_red(format!("{}\n", e));
            return Ok("Could not fetch your commands from the server.".into());
        }
    }
}
