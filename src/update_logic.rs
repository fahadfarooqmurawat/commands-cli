use std::{fs, process::Command};

pub async fn download_and_install(url: &str, install_command: &str) {
    let file_path = "/tmp/command-cli-package";
    let response = reqwest::get(url).await.expect("Failed to download");

    fs::write(
        file_path,
        response
            .bytes()
            .await
            .expect("Failed to read package bytes"),
    )
    .expect("Failed to save package file");

    println!("Installing...");
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", install_command, file_path))
        .status()
        .expect("Failed to execute install command");

    if status.success() {
        println!("Successfully updated!");
    } else {
        eprintln!("Failed to install the package.");
    }

    // Cleanup
    let _ = fs::remove_file(file_path);
}
