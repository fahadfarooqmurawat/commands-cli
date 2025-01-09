use rpassword::read_password;
use std::io::{self, Write};

pub fn read_text_from_terminal(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    email.trim().to_string()
}

pub fn read_password_from_terminal(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    read_password().unwrap()
}

pub fn read_number_from_terminal(prompt: &str, min: usize, max: usize) -> Result<usize, String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse() {
        Err(_e) => return Err("Invalid number".into()),
        Ok(index) => {
            if index >= min && index <= max {
                return Ok(index);
            } else {
                return Err("Invalid number".into());
            }
        }
    };
}
