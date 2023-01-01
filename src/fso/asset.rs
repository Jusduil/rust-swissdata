//! [Asset] represent a FSO Datasource
use std::io::Read;
use std::path::PathBuf;

use crate::tools::Downloader;

/// Type for id of FSO asset
pub type AssetId = u32;
/// FSO asset structure, for download or citation
pub struct Asset(AssetId);
impl Asset {
    /// Url for download data
    pub fn url_data(&self) -> String {
        let id = self.0;
        format!("https://dam-api.bfs.admin.ch/hub/api/dam/assets/{id}/master")
    }

    /// Url for BibTeX reference for citation of data source (can be mandatory
    /// by law)
    pub fn url_bibtex(&self) -> String {
        let id = self.0;
        format!("https://dam-api.bfs.admin.ch/hub/api/dam/assets/{id}/bibtex")
    }

    /// Url for RIS reference for citation of data source (can be mandatory by
    /// law)
    pub fn url_ris(&self) -> String {
        let id = self.0;
        format!("https://dam-api.bfs.admin.ch/hub/api/dam/assets/{id}/ris")
    }

    /// Download data in a file and return the path of file
    pub fn data_file<D>(&self, downloader: D) -> Result<PathBuf, Box<dyn std::error::Error>>
    where
        D: Downloader,
    {
        downloader.cache_get(&self.url_data())
    }

    /// Download bibtex
    pub fn bibtex<D>(&self, downloader: D) -> Result<String, Box<dyn std::error::Error>>
    where
        D: Downloader,
    {
        let mut buffer = String::new();
        downloader
            .http_get(&self.url_bibtex())?
            .read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

impl From<AssetId> for Asset {
    fn from(id: AssetId) -> Self {
        Self(id)
    }
}
