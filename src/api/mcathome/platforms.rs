use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlatformInfo {
    pub id: i32,
    pub name: String,

    #[serde(rename = "detectorBinary")]
    pub detector_binary: BinaryInfo,
}

#[derive(Debug, Deserialize)]
pub struct BinaryInfo {
    pub id: i64,
    pub checksum: String,

    #[serde(rename = "downloadURL")]
    pub download_url: String,
}

pub type PlatformListResponse = Vec<PlatformInfo>;