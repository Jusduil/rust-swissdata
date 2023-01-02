//! Struct represent meta information about dataset

use std::fmt;

use super::message::{Link, Translated};

/// Meta information about dataset
pub struct Meta<S> {
    /// Supported language of data (french, english, ...).
    ///
    /// If None data is language independant.
    pub lang: Option<Vec<String>>,

    /// Editor of datased
    pub editor: Translated<Link<S>>,

    /// Copyright information about dataset
    pub copyright: Translated<Link<S>>,

    /// Terms of use
    pub terms: Translated<Link<S>>,

    /// Terms of use (WARNING: Same mistake are possible, please check with link
    /// on [Meta::terms] before use dataset, thanks)
    pub terms_automatic: Terms,

    /// Citation for this data
    pub citations: Translated<Citation>,
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

/// Represent citation of data
#[derive(Default, Debug)]
pub struct Citation {
    bibtex: Option<String>,
    ris: Option<String>,
}
impl Citation {
    /// Get bibTeX for cite dataset
    pub fn bibtex(&self) -> Option<String> {
        self.bibtex.clone()
    }

    /// Get RIS for cite dataset
    pub fn ris(&self) -> Option<String> {
        self.ris.clone()
    }
}
/// Resume terms of use of data
#[derive(Debug)]
pub struct Terms {
    /// Data can be royalty-free use for commercial use
    pub free_commercial_use: bool,
    /// Data can be royalty-free use for non-commercial use
    pub free_noncommercial_use: bool,
    /// Citation is mandatory when data use
    ///
    /// Recommands: ALWAYS cite your data
    pub citation_mandatory: bool,
}
