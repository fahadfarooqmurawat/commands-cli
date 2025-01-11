use std::env;

pub fn is_root_user() -> bool {
    env::var("USER").unwrap_or_default() == "root"
}
