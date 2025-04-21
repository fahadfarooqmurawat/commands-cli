use crate::{
    constants::{API_URL, APP_KEY},
    objects::{response_error::ResponseError, response_post_login::ResponsePostLogin},
};
use reqwest::Client;

pub async fn post_login(email: String, password: String) -> Result<ResponsePostLogin, String> {
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
            .json::<ResponsePostLogin>()
            .await
            .map_err(|e| format!("Bad response from the server: {}", e))
    } else {
        response
            .json::<ResponseError>()
            .await
            .map(|err_message| Err(err_message.message))
            .unwrap_or_else(|_| Err(format!("Failed to login. status={}", status)))
    }
}
