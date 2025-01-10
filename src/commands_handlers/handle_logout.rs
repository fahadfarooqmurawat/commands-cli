use crate::{
    constants::{FOLDER_NAME, TOKEN_FILE, USER_FILE},
    services::file_io::delete_file,
};

pub fn handle_logout() -> Result<(), String> {
    let _ = delete_file(FOLDER_NAME, USER_FILE);
    let _ = delete_file(FOLDER_NAME, TOKEN_FILE);

    println!("Logged out");

    Ok(())
}
