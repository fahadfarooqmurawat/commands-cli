use core::fmt;
use std::{env, fs};

pub enum SupportedOS {
    Win,
    Deb,
    Rpm,
}

impl fmt::Display for SupportedOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedOS::Win => write!(f, "Windows"),
            SupportedOS::Deb => write!(f, "Linux (debian)"),
            SupportedOS::Rpm => write!(f, "Linux (rpm)"),
        }
    }
}

pub fn detect_os() -> Result<SupportedOS, String> {
    let os = env::consts::OS;
    let _arch = env::consts::ARCH;

    if os == "windows" {
        return Ok(SupportedOS::Win);
    }

    if os == "linux" {
        if is_debian_based() {
            return Ok(SupportedOS::Deb);
        }

        if is_rpm_based() {
            return Ok(SupportedOS::Rpm);
        }
    }

    return Err("Unsupported OS".into());
}

pub fn is_debian_based() -> bool {
    fs::metadata("/etc/debian_version").is_ok()
}

pub fn is_rpm_based() -> bool {
    fs::metadata("/etc/redhat-release").is_ok()
}
