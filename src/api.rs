use crate::{command::Command, user::User};
use reqwest::Client;
use serde::Deserialize;
use std::env;

const API_URL: &'static str = env!("API_URL");
const APP_KEY: &'static str = env!("APP_KEY");

#[derive(Deserialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

impl LoginResponse {
    pub fn get_token(&self) -> &str {
        &self.token
    }
    pub fn get_user(&self) -> &User {
        &self.user
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionResponse {
    pub is_latest: bool,
    pub is_compatible: bool,
    pub latest_version: String,
}

#[derive(Deserialize, Debug)]
pub struct VersionDownloadUrls {
    pub msi: String,
    pub deb: String,
    pub rpm: String,
}

pub async fn make_login_request(email: String, password: String) -> Result<LoginResponse, String> {
    let client = Client::new();

    let response = client
        .post(format!("{}/login", API_URL))
        .header("x-api-key", APP_KEY)
        .json(&serde_json::json!({ "user_email": email, "user_password": password }))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if response.status().is_success() {
        response
            .json::<LoginResponse>()
            .await
            .map_err(|e| format!("Failed to parse response JSON: {}", e))
    } else {
        Err("Invalid Credentials".into())
    }
}

pub async fn make_version_request(version: &str) -> Result<VersionResponse, String> {
    let client = Client::new();

    let response = client
        .get(format!("{}/cli/latest_version/check", API_URL))
        .header("x-api-key", APP_KEY)
        .query(&[("version", version)])
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if response.status().is_success() {
        response
            .json::<VersionResponse>()
            .await
            .map_err(|e| format!("Failed to parse response JSON: {}", e))
    } else {
        Err("Failed to fetch latest cli version".into())
    }
}

pub async fn get_version_download_urls(version: &str) -> Result<VersionDownloadUrls, String> {
    let client = Client::new();

    let response = client
        .get(format!("{}/cli/download_url", API_URL))
        .header("x-api-key", APP_KEY)
        .query(&[("version", version)])
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if response.status().is_success() {
        response
            .json::<VersionDownloadUrls>()
            .await
            .map_err(|e| format!("Failed to parse response JSON: {}", e))
    } else {
        Err("Failed to fetch download urls".into())
    }
}

pub async fn make_search_request(
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

    if response.status().is_success() {
        response
            .json::<Vec<Command>>()
            .await
            .map_err(|e| format!("Failed to parse response JSON: {}", e))
    } else {
        Err(format!("Request failed: {}", response.status()))
    }
}
