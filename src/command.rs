use core::fmt;
use serde::Deserialize;

const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const WHITE: &str = "\x1b[0m";

#[derive(Deserialize, Debug)]
pub struct Command {
    command: String,
    description: String,
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}\t{}{}{}",
            GREEN, self.command, BLUE, self.description, WHITE
        )
    }
}
