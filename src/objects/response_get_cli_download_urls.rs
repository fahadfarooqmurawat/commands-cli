use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseGetCliDownloadUrls {
    pub msi: String,
    pub deb: String,
    pub rpm: String,
}
