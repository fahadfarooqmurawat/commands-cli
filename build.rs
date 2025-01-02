use std::path::PathBuf;

fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let is_wsl = std::env::var("WSL_DISTRO_NAME").is_ok();

    let env_file = match profile.as_str() {
        "release" => PathBuf::from(".env.release"),
        _ => match is_wsl {
            true => PathBuf::from(".env.dev.wsl"),
            false => PathBuf::from(".env.dev"),
        },
    };

    let config = dotenv_build::Config {
        filename: &env_file,
        ..Default::default()
    };

    dotenv_build::output(config).unwrap();
}
