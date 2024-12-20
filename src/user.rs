use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    user_id: u32,
    user_name: String,
    user_email: String,
}

impl User {
    pub fn get_id(&self) -> u32 {
        return self.user_id;
    }
    pub fn get_name(&self) -> &str {
        return &self.user_name;
    }
    pub fn get_email(&self) -> &str {
        return &self.user_email;
    }
}
