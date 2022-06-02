use std::path::Path;

use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::{
    fs::{self, File},
    io::{self, AsyncReadExt},
};

#[derive(Debug, Deserialize)]
pub struct Download {
    url: String,
    checksums: Vec<Checksum>,
}

#[derive(Debug, Deserialize)]
pub struct Checksum {
    algorithm: String,
    value: String,
}

impl Checksum {
    pub fn new(algorithm: &str, value: &str) -> Checksum {
        Checksum {
            algorithm: algorithm.to_string(),
            value: value.to_string(),
        }
    }

    fn verify(&self, data: &[u8]) -> bool {
        match self.algorithm.as_ref() {
            "sha256" => {
                let mut algo = Sha256::new();
                algo.update(data);
                let hash = algo.finalize();
                format!("{:x}", hash) == self.value
            }
            _ => false,
        }
    }
}

impl Download {
    pub fn new(url: &str, checksums: Vec<Checksum>) -> Download {
        Download {
            url: url.to_string(),
            checksums,
        }
    }

    fn verify(&self, data: &[u8]) -> bool {
        for checksum in &self.checksums {
            if checksum.verify(data) {
                return true;
            }
        }
        false
    }

    pub async fn download(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let resp = reqwest::get(self.url.as_str())
            .await?
            .bytes()
            .await?
            .to_vec();
        if self.verify(&resp) {
            Ok(resp)
        } else {
            Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Checksum verification failed",
            )))
        }
    }

    pub async fn download_to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if path.exists() {
            // Verify the file
            let mut file = File::open(path).await?;
            let mut data = Vec::new();
            file.read_to_end(&mut data).await?;

            if self.verify(&data) {
                return Ok(());
            }
        }

        // Fallback to download
        if let Ok(data) = self.download().await {
            let mut file = File::create(&path).await?;
            io::copy(&mut data.as_slice(), &mut file).await?;
            file.sync_data().await?;
            Ok(())
        } else {
            Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Download failed",
            )))
        }
    }
}