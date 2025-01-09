use serde::Deserialize;

use super::user::User;

#[derive(Deserialize)]
pub struct ResponseLogin {
    token: String,
    user: User,
}

impl ResponseLogin {
    pub fn get_token(&self) -> &str {
        &self.token
    }
    pub fn get_user(&self) -> &User {
        &self.user
    }
}
