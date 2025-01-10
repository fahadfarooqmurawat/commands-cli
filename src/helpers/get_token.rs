use crate::{
    constants::{FOLDER_NAME, TOKEN_FILE},
    services::file_io::read_text_from_file,
};

pub fn get_token() -> std::io::Result<String> {
    let token = read_text_from_file(FOLDER_NAME, TOKEN_FILE)?;

    return Ok(token);
}
