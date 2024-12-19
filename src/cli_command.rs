use crate::cli_opts::CliOpts;

#[derive(Debug)]
pub enum CliCommand {
    Login,
    Search(String),
}

impl TryFrom<CliOpts> for CliCommand {
    type Error = String;

    fn try_from(value: CliOpts) -> Result<Self, Self::Error> {
        if value.args.is_empty() {
            return Err("No command provided".to_string());
        }

        match value.args[0].as_str() {
            "login" => Ok(CliCommand::Login),
            "search" => {
                let search_phrase = value
                    .args
                    .iter()
                    .skip(1)
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(" ");

                Ok(CliCommand::Search(search_phrase))
            }
            _ => Ok(CliCommand::Search(value.args.join(" "))),
        }
    }
}
