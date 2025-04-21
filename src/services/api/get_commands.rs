use crate::{
    constants::{API_URL, APP_KEY},
    objects::{
        response_error::ResponseError, response_get_commands::ResponseGetCommands, user::User,
    },
};
use reqwest::Client;

pub async fn get_commands(user: &User, jwt: String) -> Result<ResponseGetCommands, String> {
    let client = Client::new();

    let response = client
        .get(format!("{}/command", API_URL))
        .header("x-api-key", APP_KEY)
        .header("jwt", jwt)
        .query(&[("fk_user_id", user.get_id().to_string())])
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        response
            .json::<ResponseGetCommands>()
            .await
            .map_err(|e| format!("Bad response from the server: {}", e))
    } else {
        response
            .json::<ResponseError>()
            .await
            .map(|err_message| Err(err_message.message))
            .unwrap_or_else(|_| Err(format!("Failed to fetch commands. status={}", status)))
    }
}
