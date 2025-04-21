use crate::{
    objects::user::User,
    services::{api::get_commands::get_commands, sqlite::sync_commands},
};

pub async fn fetch_and_sync_commands(user: &User, token: String) -> String {
    if let Ok(commands) = get_commands(user, token).await {
        if let Ok(()) = sync_commands(&commands) {
            return format!(
                "Your {} commands are now available offline as well",
                commands.len()
            );
        } else {
            return "Failed to sync. Your commands are not available offline.".into();
        }
    } else {
        return "Could not fetch your commands from the server.".into();
    }
}
