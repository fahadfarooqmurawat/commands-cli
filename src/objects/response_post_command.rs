use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResponsePostCommand {
    pub success: bool,
}
