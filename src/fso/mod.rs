//! - FSO - Federal Statistical Office
//! - BFS - Bundesamt für Statistik
//! - OFS - Office fédéral de la statistique
//! - UST - Ufficia federale di statistica
pub mod communes;
//pub mod communes_historical;

pub mod asset;

use internal::*;
mod internal {
    use crate::tools::message::{Link, Translated};
    type LinkedMessage<T> = Translated<Link<T>>;
    pub fn editor() -> LinkedMessage<&'static str> {
        [
            (
                "en",
                "Federal Statistical Office",
                "https://www.bfs.admin.ch/bfs/en/home.html",
            ),
            (
                "de",
                "Bundesamt für Statistik",
                "https://www.bfs.admin.ch/bfs/de/home.html",
            ),
            (
                "fr",
                "Office fédéral de la statistique",
                "https://www.bfs.admin.ch/bfs/fr/home.html",
            ),
            (
                "it",
                "Ufficio federale di statistica",
                "https://www.bfs.admin.ch/bfs/it/home.html",
            ),
            (
                "rm",
                "Uffizi federal da statistica",
                "https://www.bfs.admin.ch/bfs/rm/home.html",
            ),
        ]
        .into_iter()
        .collect()
    }

    pub fn copyright() -> LinkedMessage<&'static str> {
        [
            (
                "en",
                "Federal Statistical Office - Legal framework",
                "https://www.bfs.admin.ch/bfs/en/home/fso/swiss-federal-statistical-office/legal-framework.html",
            ),
            (
                "de",
                "Bundesamt für Statistik - Rechtliche Hinweise",
                "https://www.bfs.admin.ch/bfs/de/home/bfs/bundesamt-statistik/rechtliche-hinweise.html",
            ),
            (
                "fr",
                "Office fédéral de la statistique - Informations juridiques",
                "https://www.bfs.admin.ch/bfs/fr/home/ofs/office-federal-statistique/informations-juridiques.html",
            ),
            (
                "it",
                "Ufficio federale di statistica - Basi legali",
                "https://www.bfs.admin.ch/bfs/it/home/ust/ufficio-federale-statistica/basi-legali.html",
            ),
        ]
        .into_iter()
        .collect()
    }
    pub fn terms<S>(title: S) -> LinkedMessage<S>
    where
        S: From<&'static str> + Default + AsRef<str> + Clone,
    {
        [
            (
                "en".into(),
                title.clone(),
                "https://www.bfs.admin.ch/bfs/en/home/fso/swiss-federal-statistical-office/terms-of-use.html".into(),
            ),
            (
                "de".into(),
                title.clone(),
                "https://www.bfs.admin.ch/bfs/de/home/bfs/bundesamt-statistik/nutzungsbedingungen.html".into(),
            ),
            (
                "fr".into(),
                title.clone(),
                "https://www.bfs.admin.ch/bfs/fr/home/ofs/office-federal-statistique/conditions-utilisation.html".into(),
            ),
            (
                "it".into(),
                title.clone(),
                "https://www.bfs.admin.ch/bfs/it/home/ust/ufficio-federale-statistica/condizioni-uso.html".into(),
            ),
        ]
        .into_iter()
        .collect()
    }
}
