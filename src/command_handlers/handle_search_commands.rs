use crate::{
    helpers::{fetch_and_sync_commands::fetch_and_sync_commands, token::get_token, user::get_user},
    services::sqlite::search_commands,
    utils::{
        copy_to_clipboard::copy_to_clipboard,
        read_from_terminal::{
            confirm_from_terminal, read_digit_from_terminal, read_number_from_terminal,
        },
        write_in_color::write_in_yellow,
    },
};

pub async fn handle_search_commands(query: Vec<String>) -> Result<(), String> {
    let user = get_user()?;
    let token = get_token()?;

    // TODO: make head request to get the etag
    // if required, fetch latest commands from the server
    // then upsert thm into the database
    // then proceed with the search
    // for now, just fetch command from the server,
    // upsert them into database and proceed

    fetch_and_sync_commands(&user, token).await;

    let commands = search_commands(&query)?;

    if commands.is_empty() {
        write_in_yellow("No commands found\n".into());

        return Ok(());
    }

    if commands.len() == 1 {
        println!("{}", commands[0]);
        copy_to_clipboard(commands[0].get_command().into());

        return Ok(());
    }

    if commands.len() <= 10 {
        for (index, command) in commands.iter().enumerate() {
            println!("{}: {}", index, command);
        }

        let selected_command = match read_digit_from_terminal(
            &format!("Please enter a number from 0 to {}: ", commands.len() - 1),
            0,
            commands.len() - 1,
        ) {
            Err(e) => return Err(e),
            Ok(index) => &commands[index],
        };

        println!("{}", selected_command);
        copy_to_clipboard(selected_command.get_command().into());

        return Ok(());
    }

    let confirm = confirm_from_terminal(&format!(
        "{} commands found, are you sure you want to print all of them on the console?",
        commands.len()
    ));

    if !confirm {
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
