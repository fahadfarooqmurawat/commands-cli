use crate::{
    constants::{API_URL, APP_KEY},
    helpers::last_updated::save_last_updated,
    objects::{
        response_error::ResponseError, response_get_commands::ResponseGetCommands, user::User,
    },
};
use reqwest::{Client, StatusCode};

pub async fn get_commands(
    user: &User,
    jwt: String,
    last_updated: Option<String>,
) -> Result<Option<ResponseGetCommands>, String> {
    let client = Client::new();

    let mut request = client
        .get(format!("{}/command", API_URL))
        .header("x-api-key", APP_KEY)
        .header("jwt", jwt)
        .query(&[("fk_user_id", user.get_id().to_string())]);

    if let Some(last_updated) = last_updated {
        request = request.header("If-Modified-Since", last_updated);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = response.status();

    match status {
        StatusCode::OK => {
            let last_modified = response
                .headers()
                .get("Last-Modified")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
                .unwrap();

            save_last_updated(&last_modified)?;

            let commands = response
                .json::<ResponseGetCommands>()
                .await
                .map_err(|e| format!("Bad response from the server: {}", e))?;

            Ok(Some(commands))
        }
        StatusCode::NOT_MODIFIED => Ok(None),
        _ => response
            .json::<ResponseError>()
            .await
            .map(|err_message| Err(err_message.message))
            .unwrap_or_else(|_| Err(format!("Failed to fetch commands. status={}", status))),
    }
}
