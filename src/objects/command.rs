use core::fmt;
use serde::Deserialize;

use crate::utils::write_in_color::{write_in_blue, write_in_green};

#[derive(Deserialize, Debug)]
pub struct Command {
    pub command_id: u32,
    pub command: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Command {
    pub fn get_id(&self) -> &u32 {
        &self.command_id
    }
    pub fn get_command(&self) -> &str {
        &self.command
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_created_at(&self) -> &str {
        &self.created_at
    }
    pub fn get_updated_at(&self) -> &str {
        &self.updated_at
    }
}

impl fmt::Display for Command {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_in_green(format!("{}\t", self.command));
        write_in_blue(format!("{}", self.description));

        Ok(())
    }
}
