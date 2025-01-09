use crate::{
    helpers::{get_token::get_token, get_user::get_user},
    services::api::request_get_search,
    utils::{
        copy_to_clipboard::copy_to_clipboard, read_from_terminal::read_number_from_terminal,
        write_in_color::write_in_yellow,
    },
};

pub async fn handle_search(query: Vec<String>) -> Result<(), String> {
    let user = get_user().unwrap();
    let token = get_token().unwrap();

    let search_text = query.join(" ");

    let commands = match request_get_search(&user, token, search_text).await {
        Err(e) => return Err(e),
        Ok(commands) => commands,
    };

    if commands.is_empty() {
        write_in_yellow("No commands found".into());
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

    let selected_command = match read_number_from_terminal(
        &format!("Please enter a number from 0 to {}: ", commands.len() - 1),
        0,
        commands.len() - 1,
    ) {
        Err(e) => return Err(e),
        Ok(index) => &commands[index],
    };

    println!("{}", selected_command.get_command());
    copy_to_clipboard(selected_command.get_command().into());

    Ok(())
}
