use std::{env, fs, process::Command};

pub async fn download_and_install(url: &str, install_command: &str) {
    let mut temp_dir = env::temp_dir();
    temp_dir.push("command-cli-package");

    let file_path = temp_dir.to_str().expect("Failed to create temp file path");
    println!("Temporary file path: {}", file_path);

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
    let status = if cfg!(target_os = "windows") {
      // Use msiexec for Windows MSI installers
      Command::new("msiexec")
          .arg("/i")
          .arg(file_path)
          .status()
          .expect("Failed to execute install command")
  } else {
      // Use shell commands for Linux (RPM or DEB)
      Command::new("sh")
          .arg("-c")
          .arg(format!("{} {}", install_command, file_path))
          .status()
          .expect("Failed to execute install command")
  };

    if status.success() {
        println!("Successfully updated!");
    } else {
        eprintln!("Failed to install the package.");
    }

    // Cleanup
    let _ = fs::remove_file(file_path);
}
