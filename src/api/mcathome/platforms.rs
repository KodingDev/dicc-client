use serde::Deserialize;

use crate::data::download::{Checksum, Download};

#[derive(Debug, Deserialize)]
pub struct PlatformInfo {
    pub id: i64,
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

impl BinaryInfo {
    pub fn as_download(&self) -> Download {
        Download::new(
            self.download_url.as_str(),
            vec![Checksum::new("sha256", self.checksum.as_str())],
        )
    }
}

pub type PlatformListResponse = Vec<PlatformInfo>;
