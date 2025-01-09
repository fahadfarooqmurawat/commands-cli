use crate::{
    constants::{FOLDER_NAME, TOKEN_FILE, USER_FILE},
    services::file_io::delete_file,
};

pub fn handle_logout() -> Result<(), String> {
    delete_file(FOLDER_NAME, USER_FILE).unwrap();
    delete_file(FOLDER_NAME, TOKEN_FILE).unwrap();

    println!("Logged out");

    Ok(())
}
