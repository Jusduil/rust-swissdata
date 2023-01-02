//! Struct represent meta information about dataset

use std::fmt;

use super::message::{Link, Translated};

/// Meta information about dataset
pub struct Meta<S> {
    /// Supported language of data (french, english, ...).
    ///
    /// If None data is language independant.
    pub(crate) lang: Option<Vec<String>>,

    /// Editor of datased
    pub(crate) editor: Translated<Link<S>>,

    /// Copyright information about dataset
    pub(crate) copyright: Translated<Link<S>>,

    /// Terms of use
    pub(crate) terms: Translated<Link<S>>,

    /// Terms of use (WARNING: Same mistake are possible, please check with link
    /// on [Meta::terms] before use dataset, thanks)
    pub(crate) terms_automatic: Terms,

    pub(crate) citations: Translated<Citation>,
}
impl<S> fmt::Debug for Meta<S>
where
    S: AsRef<str>,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Meta")
            .field("lang", &self.lang)
            .field("editor", &self.editor)
            .field("copyright", &self.copyright)
            .field("terms", &self.terms)
            .field("terms_automatic", &self.terms_automatic)
            .field("citations", &self.citations)
            .finish()
    }
}

#[derive(Default, Debug)]
pub struct Citation {
    bibtex: Option<String>,
    ris: Option<String>,
}
#[derive(Debug)]
pub struct Terms {
    pub(crate) free_commercial_use: bool,
    pub(crate) free_noncommercial_use: bool,
    pub(crate) citation_mandatory: bool,
}
