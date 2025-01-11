use std::{env, process::Command};

use crate::{
    constants::VERSION,
    services::api::{request_get_cli_download_urls, request_get_cli_latest_version_check},
    utils::{
        download_file::download_file,
        is_root_user::is_root_user,
        os_detection::{detect_os, SupportedOS},
        write_in_color::write_in_green,
    },
};

pub async fn handle_update() -> Result<(), String> {
    println!("Checking for updates");
    let response = request_get_cli_latest_version_check(VERSION).await?;

    if response.is_latest {
        println!("No new updates");
        return Ok(());
    }

    print!("New version available: ");
    write_in_green(format!("{}\n", response.latest_version));

    let urls = request_get_cli_download_urls(&response.latest_version).await?;
    let os = detect_os()?;
    println!("Detected OS: {}", os);

    let url = match os {
        SupportedOS::Win => urls.msi,
        SupportedOS::Deb => urls.deb,
        SupportedOS::Rpm => urls.rpm,
    };

    let mut temp_dir = env::temp_dir();
    temp_dir.push("command-cli-package");

    let file_path = temp_dir.to_str().ok_or("Failed to create temp file path")?;
    println!("Downloading new version installer at: {}", file_path);

    download_file(&url, file_path).await?;

    if cfg!(target_os = "windows") {
        println!("Opening MSI Installer");
        Command::new("msiexec")
            .arg("/i")
            .arg(file_path)
            .spawn()
            .map_err(|e| format!("Failed to run msiexec: {}", e))?;
    } else if cfg!(target_os = "linux") {
        if is_root_user() {
            return Err("Please run this command with sudo.".to_string());
        }

        println!("Opening Debian Installer with dpkg");
        Command::new("sudo")
            .arg("dpkg")
            .arg("-i")
            .arg(file_path)
            .spawn()
            .and_then(|mut child| child.wait())
            .map_err(|e| format!("Failed to run dpkg: {}", e))?;
    } else {
        println!("Opening Installer");
        Command::new("sh")
            .arg("-c")
            .arg(format!("sudo dpkg -i {}", file_path))
            .spawn()
            .and_then(|mut child| child.wait())
            .map_err(|e| format!("Failed to execute install command: {}", e))?;
    }

    return Ok(());
}
