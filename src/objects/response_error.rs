use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub message: String,
}
