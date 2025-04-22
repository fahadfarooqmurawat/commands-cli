use std::io::{self, IsTerminal, Write};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

fn write_in_color(message: &str, color: Color) -> Result<(), io::Error> {
    let color_choice = if io::stdout().is_terminal() {
        termcolor::ColorChoice::Always
    } else {
        termcolor::ColorChoice::Never
    };

    let mut stdout = StandardStream::stdout(color_choice);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to set color"))?;

    write!(stdout, "{}", message)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to write message"))?;

    stdout
        .reset()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to reset color"))?;

    Ok(())
}

fn handle_color_writing(message: &str, color: Color) -> () {
    if let Err(_err) = write_in_color(message, color) {
        println!("{}", message);
    }
}

pub fn write_in_red(message: String) -> () {
    handle_color_writing(&message, Color::Red);
}
pub fn write_in_yellow(message: String) -> () {
    handle_color_writing(&message, Color::Yellow);
}
pub fn write_in_green(message: String) -> () {
    handle_color_writing(&message, Color::Green);
}
pub fn write_in_blue(message: String) -> () {
    handle_color_writing(&message, Color::Blue);
}
pub fn write_in_white(message: String) -> () {
    handle_color_writing(&message, Color::White);
}
