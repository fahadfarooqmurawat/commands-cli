use crate::{command::Command, user::User};
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;

const API_URL: &str = "http://localhost:5004";

fn get_app_key() -> String {
    dotenv().ok();
    env!("APP_SECRET_KEY").to_string()
}

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

pub async fn make_login_request(email: String, password: String) -> Result<LoginResponse, String> {
    let client = Client::new();

    let response = client
        .post(format!("{}/login", API_URL))
        .header("x-api-key", get_app_key())
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

pub async fn make_search_request(
    user: &User,
    jwt: String,
    search_text: String,
) -> Result<Vec<Command>, String> {
    let client = Client::new();

    let response = client
        .get(format!("{}/search", API_URL))
        .header("x-api-key", get_app_key())
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
