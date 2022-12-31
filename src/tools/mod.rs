//! Module contains generic tools used by multiple dataset

pub mod downloader;
pub(crate) mod internal;
pub mod meta;

pub use downloader::Downloader;
