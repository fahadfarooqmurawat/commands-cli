use crate::helpers::user::get_user;

pub fn handle_user() -> Result<(), String> {
    let user = get_user()?;

    println!("{}", user);

    Ok(())
}
