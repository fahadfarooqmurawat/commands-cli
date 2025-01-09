use crate::{
    constants::{FOLDER_NAME, USER_FILE},
    objects::user::User,
    services::file_io::read_text_from_file,
};

pub fn get_user() -> Result<User, String> {
    let user_str = match read_text_from_file(FOLDER_NAME, USER_FILE) {
        None => return Err("Not logged in".into()),
        Some(data) => data,
    };

    let user = match serde_json::from_str::<User>(&user_str) {
        Err(_e) => return Err("Not logged in".into()),
        Ok(data) => data,
    };

    return Ok(user);
}
