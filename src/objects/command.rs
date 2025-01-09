use core::fmt;
use serde::Deserialize;

use crate::utils::write_in_color::{write_in_blue, write_in_green};

#[derive(Deserialize, Debug)]
pub struct Command {
    pub command: String,
    pub description: String,
}

impl Command {
    pub fn get_command(&self) -> &str {
        &self.command
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for Command {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_in_green(format!("{}\t", self.command));
        write_in_blue(format!("{}", self.description));

        Ok(())
    }
}
