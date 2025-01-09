use crate::{
    constants::{FOLDER_NAME, TOKEN_FILE},
    services::file_io::read_text_from_file,
};

pub fn get_token() -> Result<String, String> {
    let token = match read_text_from_file(FOLDER_NAME, TOKEN_FILE) {
        None => return Err("Not logged in".into()),
        Some(data) => data,
    };

    return Ok(token);
}
