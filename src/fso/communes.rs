//! This module parse and structure data for Commune, District and Canton on
//! Switzerland.
//!
//! - eCH-071 norm [[de][eCH-0071-de]] [[fr][eCH-0071-fr]]
//! - Some more explication
//!   - [Liste historisée des communes de la Suisse - Explication et
//!     utilisation][expl-fr]
//!   - [Historisiertes Gemeindeverzeichnis der Schweiz - Erläuterungen und
//!     Anwendungen][expl-de]
//! - Explication webpage
//!   - [Historisiertes Gemeindeverzeichnis][webexpl-de]
//!   - [Liste historisée des communes][webexpl-fr]
//!   - [Elenco storicizzato dei Comuni][webexpl-it]
//! - Data source (**FSO**: `dz-b-00.04-hgv-01`)
//!   - Historisiertes Gemeindeverzeichnis der Schweiz (TXT Format)
//!   - Liste historisée des communes de la Suisse (format TXT)
//!   - Elenco storicizzato dei Comuni della Svizzera (formato TXT)
//!   - [download][data-txt]
//!   - [Terms of use 'OPEN-BY-ASK'][terms]
//! - Alternative data source (**FSO**: `dz-b-00.04-hgv-02`) (not supported, but
//!   same content)
//!   - Historisiertes Gemeindeverzeichnis der Schweiz (XML Format)
//!   - Liste historisée des communes de la Suisse (format XML)
//!   - Elenco storicizzato dei Comuni della Svizzera (formato XML)
//!   - [download][data-xml]
//!   - [Terms of use 'OPEN-BY-ASK'][terms]
//!
//!
//! [eCH-0071-de]: https://www.ech.ch/fr/ech/ech-0071/1.1
//! [eCH-0071-fr]: https://www.ech.ch/de/ech/ech-0071/1.1
//! [webexpl-de]: https://www.bfs.admin.ch/bfs/de/home/grundlagen/agvch/historisiertes-gemeindeverzeichnis.html
//! [webexpl-fr]: https://www.bfs.admin.ch/bfs/fr/home/bases-statistiques/repertoire-officiel-communes-suisse/liste-historisee-communes.html
//! [webexpl-it]: https://www.bfs.admin.ch/bfs/it/home/basi-statistiche/elenco-ufficiale-comuni-svizzera/elenco-storicizzato-comuni.html
//! [expl-fr]: https://dam-api.bfs.admin.ch/hub/api/dam/assets/18484930/master
//! [expl-de]: https://dam-api.bfs.admin.ch/hub/api/dam/assets/18484929/master
//! [data-txt]: https://dam-api.bfs.admin.ch/hub/api/dam/assetsgt/23886071/master
//! [data-xml]: https://dam-api.bfs.admin.ch/hub/api/dam/assets/23886070/master
//! [terms]: https://www.bfs.admin.ch/bfs/en/home/fso/swiss-federal-statistical-office/terms-of-use.html

use std::collections::HashMap;
use std::error;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::marker::PhantomData;

use csv::{DeserializeRecordsIntoIter, ReaderBuilder as CsvReaderBuilder};
use encoding_rs;
use encoding_rs::ISO_8859_3 as ENCODING;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use zip::{read::ZipFile, ZipArchive};

use crate::fso::asset::{Asset, AssetId};
use crate::i_serde;
use crate::tools::Downloader;
use crate::tools::{dataset, meta};
use crate::Date;

/// FSO Asset id for TXT format
pub const TXT_ASSET_ID: AssetId = 23886071;
/// FSO Asset id for XML format
pub const XML_ASSET_ID: AssetId = 23886070;

/// FSO identifier about dataset on TXT format
pub const TXT_FSO_ID: &str = "dz-b-00.04-hgv-01";
/// FSO identifier about dataset on XML format
pub const XML_FSO_ID: &str = "dz-b-00.04-hgv-03";

/// An iterator on [Canton], [District] or [Municipality] according to T
pub type Iter<'a, T> = DeserializeRecordsIntoIter<DecodeReaderBytes<ZipFile<'a>, Vec<u8>>, T>;

/// Get the communes FSO datastore
pub fn datastore() -> Datastore {
    Datastore { _none: () }
}

/// Load data (format TXT only for now) with downloader, keep a reference to
/// zip file downloaded
#[derive(Default)]
pub struct Datastore {
    _none: (),
}
impl Datastore {
    /// Get asset for text format (multiple CSV (tab separator) in a zip)
    pub fn asset(&self) -> Asset {
        TXT_ASSET_ID.into()
    }

    /// Get asset for xml format (XML and XSD in a zip)
    pub fn asset_xml(&self) -> Asset {
        XML_ASSET_ID.into()
    }
}
impl dataset::Datastore<&'static str> for Datastore {
    type Store = Datasets;

    fn meta(&self) -> meta::Meta<&'static str> {
        meta::Meta {
            lang: None,
            editor: super::editor(),
            copyright: super::copyright(),
            terms: super::terms("OPEN-BY-ASK"),
            terms_automatic: meta::Terms {
                free_commercial_use: false,
                free_noncommercial_use: true,
                citation_mandatory: true,
            },
            citations: Default::default(),
        }
    }

    fn load<D>(&self, downloader: D) -> Result<Self::Store, Box<dyn error::Error>>
    where
        D: Downloader,
    {
        let path = self.asset().data_file(downloader)?;
        let file = File::open(path)?;
        let mut zip = ZipArchive::new(file)?;
        let zippath: HashMap<String, String> = zip
            .file_names()
            .filter_map(|name| {
                Some((
                    name.strip_prefix(TXT_FSO_ID)?
                        .strip_prefix("/1.2/")?
                        .strip_suffix(".txt")?
                        .split('_')
                        .nth(2)?
                        .into(),
                    name.into(),
                ))
            })
            .collect();

        fn zip_to_dataset<T>(
            zippath: &HashMap<String, String>,
            zip: &mut ZipArchive<File>,
            fname: &str,
        ) -> Result<Dataset<T>, Box<dyn error::Error>> {
            let fname = zippath
                .get(fname)
                .ok_or("Missing cantons file in archive")?
                .to_string();
            let mut output = "".into();
            DecodeReaderBytesBuilder::new()
                .encoding(Some(ENCODING))
                .build(zip.by_name(&fname)?)
                .read_to_string(&mut output)?;
            Ok(Dataset {
                raw: output,
                phantom: PhantomData,
            })
        }

        Ok(Self::Store {
            cantons: zip_to_dataset(&zippath, &mut zip, "KT")?,
            districts: zip_to_dataset(&zippath, &mut zip, "BEZ")?,
            municipalities: zip_to_dataset(&zippath, &mut zip, "GDE")?,
        })
    }
}

/// This struct contains all dataset can be retreive from data
pub struct Datasets {
    /// Canton / Kanton / Canton
    pub cantons: Dataset<Canton>,
    /// District / Bezirk / District
    pub districts: Dataset<District>,
    /// Municipality / Gemeinden / Commune
    pub municipalities: Dataset<Municipality>,
}

/// Represent a set of data, this is iterable
pub struct Dataset<T> {
    raw: String,
    phantom: PhantomData<T>,
}
impl<T> Dataset<T> {
    fn csv_reader_builder<'a>(
        &self,
        csvbuilder: &'a mut CsvReaderBuilder,
    ) -> &'a mut CsvReaderBuilder {
        csvbuilder
            .ascii()
            .delimiter(b'\t')
            .terminator(csv::Terminator::CRLF)
            .quoting(false)
            .has_headers(false)
    }
}
impl<'a, T> IntoIterator for &'a Dataset<T>
where
    T: for<'de> Deserialize<'de>,
{
    type IntoIter = DeserializeRecordsIntoIter<Cursor<&'a [u8]>, T>;
    type Item = Result<T, csv::Error>;

    fn into_iter(self) -> Self::IntoIter {
        let mut builder = CsvReaderBuilder::new();
        self.csv_reader_builder(&mut builder)
            .from_reader(Cursor::new(self.raw.as_bytes()))
            .into_deserialize()
    }
}
impl<T> IntoIterator for Dataset<T>
where
    T: for<'de> Deserialize<'de>,
{
    type IntoIter = DeserializeRecordsIntoIter<Cursor<Vec<u8>>, T>;
    type Item = Result<T, csv::Error>;

    fn into_iter(self) -> Self::IntoIter {
        let mut builder = CsvReaderBuilder::new();
        self.csv_reader_builder(&mut builder)
            .from_reader(Cursor::new(self.raw.into_bytes()))
            .into_deserialize()
    }
}

/// Represent a identifier of canton
pub type CantonId = u8;
/// Represent a historical identifier of district
pub type DistrictHistId = u32;
/// Represent a identifier of district
pub type DistrictId = u16;
/// Represent a historical identifier of municipality
pub type MunicipalityHistId = u32;
/// Represent a identifier of municipality
pub type MunicipalityId = u16;
/// Represent a identifier of mutation (change, admission or abolition)
pub type MutationId = u16;

/// Status of municipality. All step (municipality, canton and national) are
/// done if status is [Status::Final]
///
/// Dieses Merkmal dient der Unterscheidung von Mutationen, die alle Verfahren
/// auf Stufe Gemeinde, Kanton und Bund durchlaufen haben (1 = definitiv), und
/// denjenigen, die noch nicht alle Verfahren durchlaufen haben (0 =
/// provisorisch).
///
/// Ce caractère sert à différencier les mutations qui sont passées par toutes
/// les étapes, à l’échelon de la commune, du canton et de la Confédération (1 =
/// définitif) de celles qui n’ont pas encore franchi toutes les étapes (0 =
/// provisoire).
#[derive(Copy, Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum Status {
    /// Provisorisch / Provisoire
    Tentative = 0,
    /// Definitiv / Définitif
    Final = 1,
}
#[derive(Copy, Clone, Debug, Deserialize_repr)]
#[repr(u8)]
/// Type of municipality
pub enum MunicipalityMode {
    /// Politische Gemeinde / Commune politique
    PoliticalCommune = 11,
    /// Gemeindefreies Gebiet / Territoire non attribué à une commune
    MunicipalityFreeArea = 12,
    /// Kantonaler Seeanteil / Partie cantonale de lac
    CantonalLakePortion = 13,
}
#[derive(Copy, Clone, Debug, Deserialize_repr)]
#[repr(u8)]
/// Type of district
pub enum DistrictMode {
    /// Bezirk / District
    District = 15,
    /// Kanton ohne Bezirksunterteilung / Canton sans districts
    CantonWithoutDistricts = 16,
    /// Bezirksfreies Gebiet / Territoire non attribué à un district
    DistrictFreeArea = 17,
}
#[derive(Copy, Clone, Debug, Deserialize_repr)]
#[repr(u8)]
/// Type of action has trigg admission
pub enum AdmissionMode {
    /// Ersterfassung Gemeinde/Bezirk / Première saisie commune/district
    FirstRegistration = 20,
    /// Neugründung Gemeinde/Bezirk / Création commune/district
    Creation = 21,
    /// Namensänderung Bezirk / Changement de nom du district
    DistrictNameChange = 22,
    /// Namensänderung Gemeinde / Changement de nom de la commune
    MunicipalityNameChange = 23,
    /// Neue Bezirks-/Kantonszuteilung / Rattachement à un autre district/canton
    AttachmentToAnother = 24,
    /// Gebietsänderung Gemeinde / Modification du territoire de la commune
    TerritoryMunicipalityChange = 26,
    /// Formale Neunummerierung Gemeinde/Bezirk / Renumérotation formelle de la
    /// commune/du district
    FormalRenumbering = 27,
}
#[derive(Copy, Clone, Debug, Deserialize_repr)]
#[repr(u8)]
/// Type of action has trigg abolition
pub enum AbolitionMode {
    /// Namensänderung Bezirk / Changement de nom du district
    DistrictNameChange = 22,
    /// Namensänderung Gemeinde / Changement de nom de la commune
    MunicipalityNameChange = 23,
    /// Neue Bezirks-/Kantonszuteilung / Rattachement à un autre district/canton
    AttachmentToAnother = 24,
    /// Gebietsänderung Gemeinde / Modification du territoire de la commune
    TerritoryMunicipalityChange = 26,
    /// Formale Neunummerierung Gemeinde/Bezirk / Renumérotation formelle de la
    /// commune/du district
    FormalRenumbering = 27,
    /// Aufhebung Gemeinde/Bezirk / Radiation commune/district
    Radiation = 29,
    /// Mutation annulliert / Annulation de la mutation
    MutationAnnulled = 30,
}

/// Canton / Kanton / Canton
#[derive(Debug, Deserialize)]
pub struct Canton {
    /// Canton number / Kantonsnummer / Numéro du canton
    pub id: CantonId,
    /// Canton's abbreviation / Kantonskürzel / Abréviation du canton
    pub abbreviation: String,
    /// Canton's name / Kantonsname / Nom du canton
    pub name: String,
    #[serde(with = "i_serde::date_dd_mm_yyyyy_dotted")]
    /// Change date / Änderungsdatum / Date de modification
    pub date_of_change: Date,
}

/// Bezirk / District
#[derive(Debug, Deserialize)]
pub struct District {
    /// Historic identifier
    /// / Historisierungsnummer BEZ
    /// / Numéro d’historisation DIS
    pub hist_id: DistrictHistId,
    /// Canton identifier
    /// / Kantonsnummer
    /// / Numéro du canton
    pub canton_id: CantonId,
    /// District identifier
    /// / Bezirksnummer
    /// / Numéro du district
    pub id: DistrictId,
    /// District name
    /// / Bezirksname
    /// / Nom du district
    pub name: String,
    /// District abbreviated name
    /// / Bezirksname kurz
    /// / Nom du district en abrégé
    pub short_name: String,
    /// Entry type
    /// / Art des Eintrages
    /// / Type d’entrée
    pub entry_mode: DistrictMode,
    /// please see [Self::admission] for get a Mutation struct
    pub admission_number: MutationId,
    /// please see [Self::admission] for get a Mutation struct
    pub admission_mode: AdmissionMode,
    /// please see [Self::admission] for get a Mutation struct
    #[serde(with = "i_serde::date_dd_mm_yyyyy_dotted")]
    pub admission_date: Date,
    /// please see [Self::abolition] for get a Mutation struct
    pub abolition_number: Option<MutationId>,
    /// please see [Self::abolition] for get a Mutation struct
    pub abolition_mode: Option<AbolitionMode>,
    /// please see [Self::abolition] for get a Mutation struct
    #[serde(with = "i_serde::option_date_dd_mm_yyyyy_dotted")]
    pub abolition_date: Option<Date>,
    /// Date of the last change
    /// / Änderungsdatum
    /// / Date de modification
    #[serde(with = "i_serde::date_dd_mm_yyyyy_dotted")]
    pub date_of_change: Date,
}
impl District {
    /// Information about district admission
    pub fn admission(&self) -> Mutation<AdmissionMode> {
        Mutation {
            number: self.admission_number,
            mode: self.admission_mode,
            date: &self.admission_date,
        }
    }

    /// Information about district abolition (if is abolited)
    pub fn abolition(&self) -> Option<Mutation<AbolitionMode>> {
        Some(Mutation {
            number: self.abolition_number?,
            mode: self.abolition_mode?,
            date: self.abolition_date.as_ref()?,
        })
    }
}

#[derive(Debug, Deserialize)]
/// Municipality / Gemeinden / Commune
pub struct Municipality {
    /// Municipality historical identifier
    pub hist_id: MunicipalityHistId,
    /// District historical identifier
    pub district_hist_id: DistrictHistId,
    /// Abbreviation of canton (two letter)
    pub canton_abbreviation: String,
    /// Municipality identifier
    pub id: MunicipalityId,
    /// Municipality official name
    pub name: String,
    /// Municipality abbreviated name
    pub short_name: String,
    /// Type of municipality
    pub entry_mode: MunicipalityMode,
    /// Status of change
    pub status: Status,
    /// please see [Self::admission] for get a Mutation struct
    pub admission_number: MutationId,
    /// please see [Self::admission] for get a Mutation struct
    pub admission_mode: AdmissionMode,
    /// please see [Self::admission] for get a Mutation struct
    #[serde(with = "i_serde::date_dd_mm_yyyyy_dotted")]
    pub admission_date: Date,
    /// please see [Self::abolition] for get a Mutation struct
    pub abolition_number: Option<MutationId>,
    /// please see [Self::abolition] for get a Mutation struct
    pub abolition_mode: Option<AbolitionMode>,
    /// please see [Self::abolition] for get a Mutation struct
    #[serde(with = "i_serde::option_date_dd_mm_yyyyy_dotted")]
    pub abolition_date: Option<Date>,
    /// Date of the last change
    /// / Änderungsdatum
    /// / Date de modification
    #[serde(with = "i_serde::date_dd_mm_yyyyy_dotted")]
    pub date_of_change: Date,
}
impl Municipality {
    /// Information about district admission
    pub fn admission(&self) -> Mutation<AdmissionMode> {
        Mutation {
            number: self.admission_number,
            mode: self.admission_mode,
            date: &self.admission_date,
        }
    }

    /// Information about district abolition (if is abolited)
    pub fn abolition(&self) -> Option<Mutation<AbolitionMode>> {
        Some(Mutation {
            number: self.abolition_number?,
            mode: self.abolition_mode?,
            date: self.abolition_date.as_ref()?,
        })
    }
}

#[derive(Debug)]
/// A mutation data (admission or abolition)
pub struct Mutation<'a, Mode: Copy> {
    /// Identifier of Mutation
    pub number: MutationId,
    /// Reason of mutation
    pub mode: Mode,
    /// Date of mutation
    pub date: &'a Date,
}
