use serde::Deserialize;

use super::user::User;

#[derive(Deserialize)]
pub struct ResponsePostLogin {
    token: String,
    user: User,
}

impl ResponsePostLogin {
    pub fn get_token(&self) -> &str {
        &self.token
    }
    pub fn get_user(&self) -> &User {
        &self.user
    }
}
