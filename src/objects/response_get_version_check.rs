use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseGetVersionCheck {
    pub is_latest: bool,
    pub is_compatible: bool,
    pub latest_version: String,
}
