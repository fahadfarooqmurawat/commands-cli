use crate::{
    constants::{API_URL, APP_KEY},
    objects::{
        command::Command, response_cli_download_urls::VersionCliDownloadUrls,
        response_cli_latest_version_check::ResponseCliLatestVersionCheck,
        response_login::ResponseLogin, user::User,
    },
};
use reqwest::Client;

pub async fn request_post_login(email: String, password: String) -> Result<ResponseLogin, String> {
    let client = Client::new();

    let response = client
        .post(format!("{}/login", API_URL))
        .header("x-api-key", APP_KEY)
        .json(&serde_json::json!({ "user_email": email, "user_password": password }))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        response
            .json::<ResponseLogin>()
            .await
            .map_err(|e| format!("Bad response from the server: {}", e))
    } else {
        Err("Invalid Credentials".into())
    }
}

pub async fn request_get_cli_latest_version_check(
    version: &str,
) -> Result<ResponseCliLatestVersionCheck, String> {
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
            .json::<ResponseCliLatestVersionCheck>()
            .await
            .map_err(|e| format!("Bad response from the server: {}", e))
    } else {
        Err(format!(
            "Failed to fetch latest cli version. status={}",
            status
        ))
    }
}

pub async fn request_get_cli_download_urls(
    version: &str,
) -> Result<VersionCliDownloadUrls, String> {
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
            .json::<VersionCliDownloadUrls>()
            .await
            .map_err(|e| format!("Bad response from the server:: {}", e))
    } else {
        Err(format!("Failed to fetch download urls. status={}", status))
    }
}

pub async fn request_get_search(
    user: &User,
    jwt: String,
    search_text: String,
) -> Result<Vec<Command>, String> {
    let client = Client::new();

    let response = client
        .get(format!("{}/search", API_URL))
        .header("x-api-key", APP_KEY)
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
            .json::<Vec<Command>>()
            .await
            .map_err(|e| format!("Bad response from the server: {}", e))
    } else {
        Err(format!("Failed to fetch commands. status={}", status))
    }
}
