use std::{env, fs};

pub fn detect_os() {
    let os = env::consts::OS; // Returns "windows", "linux", "macos", etc.
    let arch = env::consts::ARCH; // Returns "x86", "x86_64", "arm", etc.

    println!("Operating System: {}", os);
    println!("Architecture: {}", arch);

    match os {
        "windows" => println!("Detected Windows"),
        "linux" => println!("Detected Linux"),
        "macos" => println!("Detected macOS"),
        _ => println!("Unsupported operating system"),
    }
}

pub fn is_debian_based() -> bool {
    fs::metadata("/etc/debian_version").is_ok()
}

pub fn is_rpm_based() -> bool {
    fs::metadata("/etc/redhat-release").is_ok()
}
