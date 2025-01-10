use crate::{
    constants::{FOLDER_NAME, USER_FILE},
    objects::user::User,
    services::file_io::read_text_from_file,
};

pub fn get_user() -> std::io::Result<User> {
    let user_str = read_text_from_file(FOLDER_NAME, USER_FILE)?;
    let user = serde_json::from_str::<User>(&user_str)?;

    return Ok(user);
}
