use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResponseAdd {
    pub success: bool,
}
