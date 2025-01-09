use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseCliLatestVersionCheck {
    pub is_latest: bool,
    pub is_compatible: bool,
    pub latest_version: String,
}
