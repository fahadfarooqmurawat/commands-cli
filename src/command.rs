use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Command {
    pub command: String,
    pub description: String,
}
