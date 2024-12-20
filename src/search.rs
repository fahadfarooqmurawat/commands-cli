use crate::{
    api::make_search_request,
    file_io::{read_token, read_user},
    utils::copy_to_clipboard,
    utils::get_number,
};

pub async fn search(search_text: String) -> Result<(), String> {
    let user = match read_user() {
        Some(user) => user,
        None => {
            println!("Not logged in");
            return Ok(());
        }
    };

    let token = match read_token() {
        Some(token) => token,
        None => {
            println!("Not loggd in");
            return Ok(());
        }
    };

    match make_search_request(&user, token, search_text).await {
        Ok(commands) => {
            if commands.is_empty() {
                println!("No commands found");
                return Ok(());
            }

            if commands.len() == 1 {
                println!("{}", commands[0]);
                copy_to_clipboard(commands[0].get_command().into());
                return Ok(());
            }

            for (index, command) in commands.iter().enumerate() {
                println!("{}: {}", index, command);
            }

            let selected_command = loop {
                match get_number(0, commands.len() - 1) {
                    Ok(index) => {
                        if index < commands.len() {
                            break &commands[index];
                        }

                        println!("Please enter a number from 0 to {}", commands.len() - 1);
                    }
                    Err(e) => {
                        println!("{}", e);
                        return Ok(());
                    }
                }
            };

            println!("{}", selected_command.get_command());
            copy_to_clipboard(selected_command.get_command().into());

            Ok(())
        }
        Err(e) => Err(e),
    }
}
