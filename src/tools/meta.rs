//! Struct represent meta information about dataset

use std::fmt;

use super::message::{Link, Translated};

/// Meta information about dataset
pub struct Meta<S> {
    /// Dataset name
    pub name: Translated<S>,

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
impl<S> Meta<S>
where
    S: fmt::Display,
{
    /// Return the localized description
    pub fn to_l10n_string(&self, lang: &str) -> String {
        let name = self.name.get_or_default(lang);
        let editor_name = self.editor.get_or_default(lang).name();
        let editor_url = self.editor.get_or_default(lang).url();
        let copyright_name = self.copyright.get_or_default(lang).name();
        let copyright_url = self.copyright.get_or_default(lang).url();
        let terms_name = self.terms.get_or_default(lang).name();
        let terms_url = self.terms.get_or_default(lang).url();
        let Terms {
            free_commercial_use: co,
            free_noncommercial_use: nc,
            citation_mandatory: cite,
        } = &self.terms_automatic;
        let citations = self.citations.get_or_default(lang);
        format!(
            "## {name}
editor: {editor_name}
copyright: {copyright_name}
terms: {terms_name}
       commercial use allowed: {co:?}
       non commercial use allowed: {nc:?}
       citations mandatory: {cite:?}
citations: {citations:?}

[editor]: {editor_url}
[copyright]: {copyright_url}
[terms]: {terms_url}"
        )
    }
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
    pub(crate) fn new<S: AsRef<str>>(bibtex: S, ris: S) -> Self {
        Self {
            bibtex: Some(bibtex.as_ref().to_string()),
            ris: Some(ris.as_ref().to_string()),
        }
    }

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
