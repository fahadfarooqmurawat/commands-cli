use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VersionCliDownloadUrls {
    pub msi: String,
    pub deb: String,
    pub rpm: String,
}
