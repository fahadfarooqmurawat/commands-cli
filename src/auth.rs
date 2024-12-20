use crate::api::make_login_request;
use crate::file_io::delete_token;
use crate::file_io::delete_user;
use crate::file_io::save_token;
use crate::file_io::save_user;
use crate::utils::get_email;
use crate::utils::get_password;

pub async fn login() -> Result<(), String> {
    let email = get_email();
    let password = get_password();

    match make_login_request(email, password).await {
        Err(e) => {
            return Err(e);
        }
        Ok(login_response) => {
            let token = login_response.get_token();
            let user = login_response.get_user();

            if let Err(e) = save_token(&token) {
                return Err(format!("Failed to save token: {}", e));
            };

            if let Err(e) = save_user(&user) {
                return Err(format!("Failed to save user: {}", e));
            };

            println!("Welcome {}", user.get_name());

            Ok(())
        }
    }
}

pub fn logout() -> Result<(), String> {
    if let Err(e) = delete_token() {
        return Err(format!("Error deleting token: {}", e));
    }

    if let Err(e) = delete_user() {
        return Err(format!("Error deleting user data: {}", e));
    }

    println!("Logged out");

    Ok(())
}
