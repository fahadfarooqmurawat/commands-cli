use crate::{
    api::{get_version_download_urls, make_version_request},
    os_detection::{detect_os, SupportedOS},
    update_logic::download_and_install,
    write_in_color::write_in_color,
};
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

pub async fn update_to_latest() -> Result<(), String> {
    let current_version = env!("CARGO_PKG_VERSION");

    println!("Checking for updates");
    match make_version_request(current_version).await {
        Ok(response) => {
            if response.is_latest {
                println!("Already latest version");
                Ok(())
            } else {
                println!(
                    "Upgrading from {} to {}",
                    current_version, response.latest_version
                );
                match get_version_download_urls(&response.latest_version).await {
                    Ok(urls) => {
                        match detect_os() {
                            Ok(os) => {
                                println!("Detected {}", os);
                                let (url, install_command) = match os {
                                    SupportedOS::Win => (urls.msi, "msiexec /i"),
                                    SupportedOS::Deb => (urls.deb, "sudo dpkg i"),
                                    SupportedOS::Rpm => (urls.rpm, "sudo rpm -i"),
                                };

                                download_and_install(&url, &install_command).await;
                            }
                            Err(err) => {
                                let _ = write_in_color(
                                    "ERROR: Failed to detect Operating System\n".into(),
                                    Color::Red,
                                );

                                return Err(err);
                            }
                        }
                        Ok(())
                    }
                    Err(_err) => {
                        let _ = write_in_color(
                            "ERROR: Failed to fetch latest CLI urls\n".into(),
                            Color::Red,
                        );

                        Err("Failed to update".into())
                    }
                }
            }
        }

        Err(_err) => {
            let _ = write_in_color(
                "ERROR: Failed to fetch latest CLI version\n".into(),
                Color::Red,
            );

            Err("Failed to update".into())
        }
    }
}
