use crate::write_in_color::write_in_color;
use core::fmt;
use serde::Deserialize;
use termcolor::Color;

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
        let _ = write_in_color(format!("{}\t", self.command), Color::Green);
        let _ = write_in_color(format!("{}", self.description), Color::Blue);

        Ok(())
    }
}
