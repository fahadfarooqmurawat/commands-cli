use crate::{
    constants::{API_URL, APP_KEY},
    objects::{
        response_error::ResponseError, response_get_cli_download_urls::ResponseGetCliDownloadUrls,
    },
};
use reqwest::Client;

pub async fn get_cli_download_urls(version: &str) -> Result<ResponseGetCliDownloadUrls, String> {
    let client = Client::new();

    let response = client
        .get(format!("{}/cli/download_url", API_URL))
        .header("x-api-key", APP_KEY)
        .query(&[("version", version)])
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        response
            .json::<ResponseGetCliDownloadUrls>()
            .await
            .map_err(|e| format!("Bad response from the server:: {}", e))
    } else {
        response
            .json::<ResponseError>()
            .await
            .map(|err_message| Err(err_message.message))
            .unwrap_or_else(|_| Err(format!("Failed to fetch download urls. status={}", status)))
    }
}
