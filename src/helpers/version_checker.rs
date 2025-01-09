use crate::{constants::VERSION, services::api::request_get_cli_latest_version_check};

pub async fn version_checker() -> Result<Option<String>, String> {
    match request_get_cli_latest_version_check(VERSION).await {
        Err(_err) => return Err(format!("ERROR: Failed to fetch latest CLI version")),
        Ok(response) => {
            if !response.is_compatible {
                return Err(format!(
                    "ERROR: Your version ({}) is not compatible with the latest version ({})",
                    VERSION, response.latest_version
                ));
            }

            if response.is_compatible && !response.is_latest {
                return Ok(Some(format!(
                    "WARNING: A new version ({}) is available\n",
                    response.latest_version
                )));
            }

            Ok(None)
        }
    }
}
