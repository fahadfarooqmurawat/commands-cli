use atty;
use core::fmt;
use serde::Deserialize;
use termcolor::{Color, ColorChoice, StandardStream, WriteColor};

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color_choice = if atty::is(atty::Stream::Stdout) {
            ColorChoice::Always
        } else {
            ColorChoice::Never
        };

        let mut stdout = StandardStream::stdout(color_choice);

        stdout
            .set_color(termcolor::ColorSpec::new().set_fg(Some(Color::Green)))
            .map_err(|_| fmt::Error)?;
        write!(f, "{}\t", self.command)?;

        stdout
            .set_color(termcolor::ColorSpec::new().set_fg(Some(Color::Blue)))
            .map_err(|_| fmt::Error)?;
        write!(f, "{}", self.description)?;

        stdout.reset().map_err(|_| fmt::Error)?;

        Ok(())
    }
}
