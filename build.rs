use std::path::PathBuf;

fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let env_file = match profile.as_str() {
        "release" => PathBuf::from(".env.release"),
        _ => PathBuf::from(".env.dev"),
    };

    let config = dotenv_build::Config {
        filename: &env_file,
        ..Default::default()
    };

    dotenv_build::output(config).unwrap();
}
