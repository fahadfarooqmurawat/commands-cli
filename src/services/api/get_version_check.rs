use crate::{
    constants::{API_URL, APP_KEY},
    objects::{response_error::ResponseError, response_get_version_check::ResponseGetVersionCheck},
};
use reqwest::Client;

pub async fn get_version_check(version: &str) -> Result<ResponseGetVersionCheck, String> {
    let client = Client::new();

    let response = client
        .get(format!("{}/cli/latest_version/check", API_URL))
        .header("x-api-key", APP_KEY)
        .query(&[("version", version)])
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        response
            .json::<ResponseGetVersionCheck>()
            .await
            .map_err(|e| format!("Bad response from the server:: {}", e))
    } else {
        response
            .json::<ResponseError>()
            .await
            .map(|err_message| Err(err_message.message))
            .unwrap_or_else(|_| Err(format!("Failed to fetch version info. status={}", status)))
    }
}
