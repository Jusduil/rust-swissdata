//! Traits represent standard interface to access data

use std::error;

use crate::tools::meta::Meta;
use crate::tools::Downloader;

/// For struct contains references to data (before downloading)
pub trait Datastore<S> {
    /// Store used for data access
    type Store;

    /// Meta information about data, (license, copyright, URL, ...)
    fn meta(&self) -> Meta<S>;

    /// Lang of data (if knowledge before downloading)
    fn availabled_lang(&self) -> Option<Vec<String>> {
        None
    }

    /// Download data with downloader and return the stor for access to this
    /// data
    fn load<D>(&self, downloader: D) -> Result<Self::Store, Box<dyn error::Error>>
    where
        D: Downloader;
}
