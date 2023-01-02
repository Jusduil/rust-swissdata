use std::error;

use crate::tools::meta::Meta;
use crate::tools::Downloader;

pub trait Datastore<S> {
    type Store;

    fn meta(&self) -> Meta<S>;
    fn availabled_lang(&self) -> Option<Vec<String>> {
        None
    }
    fn load<D>(&self, downloader: D) -> Result<Self::Store, Box<dyn error::Error>>
    where
        D: Downloader;
}
