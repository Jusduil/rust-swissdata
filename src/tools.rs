//! Module with some trait/struct use in this crate

use dirs;
use reqwest;
use std::error;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use urlencoding;

/// Downloader trait for use a custom lib for download and cache data
pub trait Downloader {
    /// Error emit when download fail
    type DownloadError: std::error::Error + 'static;
    /// Reader return after download (without cache)
    type Read: Read;
    /// default validity duration for cache
    fn default_validity(&self) -> Duration;
    /// return path of cache file for an url
    fn cache_path(&self, url: &str) -> Result<PathBuf, Box<dyn std::error::Error>>;
    /// direct download url
    fn http_get(&self, url: &str) -> Result<Self::Read, Self::DownloadError>;

    /// check if a cache is valid
    fn is_valid<P: AsRef<Path>>(&self, path: P) -> Result<bool, Box<dyn error::Error>> {
        Ok(path.as_ref().is_file()
            && path.as_ref().metadata()?.modified()? + self.default_validity() > SystemTime::now())
    }

    /// Get path with valid data for url (download if required)
    fn cache_get(&self, url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = self.cache_path(url)?;
        dbg!(url, &path, !self.is_valid(&path)?);
        if !self.is_valid(&path)? {
            let mut result = self.http_get(url)?;
            let mut file = File::create(&path)?;
            io::copy(&mut result, &mut file)?;
        }
        Ok(path)
    }
}

impl Downloader for &reqwest::blocking::Client {
    type DownloadError = reqwest::Error;
    type Read = reqwest::blocking::Response;
    fn default_validity(&self) -> Duration {
        Duration::new(60 * 60 * 24, 0)
    }
    fn cache_path(&self, url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = dirs::cache_dir()
            .ok_or("Can't determine cache directory")?
            .join("rust-swissdata");
        fs::create_dir_all(&path)?;
        Ok(path.join(urlencoding::encode(url).into_owned()))
    }
    fn http_get(&self, url: &str) -> Result<Self::Read, Self::DownloadError> {
        dbg!(url);
        Ok(self.get(url).send()?.error_for_status()?)
    }
}
