use crate::{
    constants::{API_URL, APP_KEY},
    objects::{
        response_error::ResponseError, response_post_command::ResponsePostCommand, user::User,
    },
};
use reqwest::Client;

pub async fn post_command(
    user: &User,
    jwt: String,
    command: String,
    description: String,
) -> Result<ResponsePostCommand, String> {
    let client = Client::new();

    let response = client
        .post(format!("{}/command", API_URL))
        .header("x-api-key", APP_KEY)
        .header("jwt", jwt)
        .json(&serde_json::json!({"fk_user_id": user.get_id(), "command": command, "description": description }))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        response
            .json::<ResponsePostCommand>()
            .await
            .map_err(|e| format!("Bad response from the server: {}", e))
    } else {
        response
            .json::<ResponseError>()
            .await
            .map(|err_message| Err(err_message.message))
            .unwrap_or_else(|_| Err(format!("Failed to add command. status={}", status)))
    }
}
