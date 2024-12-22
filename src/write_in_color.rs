use std::io::{self, Write};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor}; // Import Write from std::io

pub fn write_in_color(message: String, color: Color) -> Result<(), io::Error> {
    let color_choice = if atty::is(atty::Stream::Stdout) {
        termcolor::ColorChoice::Always
    } else {
        termcolor::ColorChoice::Never
    };

    let mut stdout = StandardStream::stdout(color_choice);

    // Set color
    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to set color"))?;

    // Write the message
    write!(stdout, "{}", message)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to write message"))?;

    // Reset color
    stdout
        .reset()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to reset color"))?;

    Ok(())
}
