use core::fmt;

use serde::{Deserialize, Serialize};

use crate::utils::write_in_color::{write_in_green, write_in_white};

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

impl fmt::Display for User {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write_in_green(format!("{}", self.user_name));
        write_in_white(format!(" ({})", self.user_email));

        Ok(())
    }
}
