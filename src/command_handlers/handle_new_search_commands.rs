use crate::{
    helpers::{token::get_token, user::get_user},
    services::api::get_search_commands::get_search_commands,
    utils::{
        copy_to_clipboard::copy_to_clipboard, read_from_terminal::read_number_from_terminal,
        write_in_color::write_in_yellow,
    },
};

pub async fn handle_new_search_commands(query: Vec<String>) -> Result<(), String> {
    let user = get_user()?;
    let token = get_token()?;

    let search_text = query.join(" ");

    let commands = get_search_commands(&user, token, search_text).await?;

    if commands.is_empty() {
        write_in_yellow("No commands found\n".into());
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

    println!("{}", selected_command);

    copy_to_clipboard(selected_command.get_command().into());

    Ok(())
}
