use rpassword::read_password;
use std::io::{self, Write};

pub fn get_email() -> String {
    print!("Email: ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    email.trim().to_string()
}

pub fn get_password() -> String {
    print!("Password: ");
    io::stdout().flush().unwrap();
    read_password().unwrap()
}

pub fn get_number(from: usize, to: usize) -> Result<usize, String> {
    print!("Enter Command Number [{}-{}]: ", from, to);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse() {
        Ok(index) => {
            return Ok(index);
        }
        _ => {
            return Err("Invalid input".into());
        }
    };
}
