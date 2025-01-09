use crate::helpers::get_user::get_user;

pub fn handle_user() -> Result<(), String> {
    let user = get_user().unwrap();

    println!("{}", user);

    Ok(())
}
