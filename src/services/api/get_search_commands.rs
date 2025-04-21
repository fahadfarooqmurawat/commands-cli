use crate::{
    constants::{API_URL, APP_KEY, VERSION},
    objects::{
        response_error::ResponseError, response_get_search_commands::ResponseGetSearchCommands,
        user::User,
    },
};
use reqwest::Client;

pub async fn get_search_commands(
    user: &User,
    jwt: String,
    search_text: String,
) -> Result<ResponseGetSearchCommands, String> {
    let client = Client::new();

    let response = client
        .get(format!("{}/search", API_URL))
        .header("x-api-key", APP_KEY)
        .header("cli-version", VERSION)
        .header("jwt", jwt)
        .query(&[
            ("fk_user_id", user.get_id().to_string()),
            ("search", search_text),
        ])
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        response
            .json::<ResponseGetSearchCommands>()
            .await
            .map_err(|e| format!("Bad response from the server: {}", e))
    } else {
        response
            .json::<ResponseError>()
            .await
            .map(|err_message| Err(err_message.message))
            .unwrap_or_else(|_| Err(format!("Failed to search for commands. status={}", status)))
    }
}
