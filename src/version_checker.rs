use crate::{api::make_version_request, write_in_color::write_in_color};
use termcolor::Color;

pub async fn check_latest_version() -> Result<(), String> {
    let version = env!("CARGO_PKG_VERSION");

    match make_version_request(version).await {
        Ok(response) => {
            if !response.is_compatible {
                let _ = write_in_color(
                    format!(
                        "ERROR: Your version ({}) is not compatible with the latest version ({})",
                        version, response.latest_version
                    ),
                    Color::Red,
                );

                return Err("Version not compatible".to_string());
            }

            if response.is_compatible && !response.is_latest {
                let _ = write_in_color(
                    format!(
                        "WARNING: A new version ({}) is available\n",
                        response.latest_version
                    ),
                    Color::Yellow,
                );
            }

            Ok(())
        }

        Err(_err) => {
            let _ = write_in_color(
                "ERROR: Failed to fetch latest CLI version".into(),
                Color::Red,
            );

            Err("Failed to fetch version".into())
        }
    }
}
