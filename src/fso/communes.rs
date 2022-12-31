//! This module parse and structure data for Commune, District and Canton on Switzerland.
//!
//! - eCH-071 norm [[de][eCH-0071-de]] [[fr][eCH-0071-fr]]
//! - Some more explication
//!   - [Liste historisée des communes de la Suisse - Explication et utilisation][expl-fr]
//!   - [Historisiertes Gemeindeverzeichnis der Schweiz - Erläuterungen und Anwendungen][expl-de]
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
//! - Alternative data source (**FSO**: `dz-b-00.04-hgv-02`) (not supported, but same content)
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

use crate::fso::asset::{Asset, AssetId};
use crate::tools::Downloader;
use crate::Date;
use csv::{DeserializeRecordsIntoIter, ReaderBuilder as CsvReaderBuilder};
use encoding_rs;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::collections::HashMap;
use std::error;
use std::fs::File;
use zip::{read::ZipFile, ZipArchive};

use encoding_rs::ISO_8859_3 as ENCODING;

/// FSO Asset id for TXT format
pub const TXT_ASSET_ID: AssetId = 23886071;
/// FSO Asset id for XML format
pub const XML_ASSET_ID: AssetId = 23886070;

/// FSO identifier about dataset on TXT format
pub const TXT_FSO_ID: &'static str = "dz-b-00.04-hgv-01";
/// FSO identifier about dataset on XML format
pub const XML_FSO_ID: &'static str = "dz-b-00.04-hgv-03";

/// An iterator on [Canton], [District] or [Municipality] according to T
pub type Iter<'a, T> = DeserializeRecordsIntoIter<DecodeReaderBytes<ZipFile<'a>, Vec<u8>>, T>;

/// Get asset for text format (multiple CSV (tab separator) in a zip)
pub fn asset_txt() -> Asset {
    TXT_ASSET_ID.into()
}
/// Get asset for xml format (XML and XSD in a zip)
pub fn asset_xml() -> Asset {
    XML_ASSET_ID.into()
}

/// Struct for acess data stored in zip archive
pub struct DataStore {
    zip: ZipArchive<File>,
    cantons: String,
    districts: String,
    municipalities: String,
}
impl DataStore {
    /// Load data (format TXT only for now) with downloader, keep a reference to zip file
    /// downloaded
    pub fn load<D>(downloader: D) -> Result<Self, Box<dyn error::Error>>
    where
        D: Downloader,
    {
        let path = asset_txt().data_file(downloader)?;
        let file = File::open(path)?;
        let zip = ZipArchive::new(file)?;
        let zippath: HashMap<_, _> = zip
            .file_names()
            .filter_map(|name| {
                Some((
                    name.strip_prefix(TXT_FSO_ID)?
                        .strip_prefix("/1.2/")?
                        .strip_suffix(".txt")?
                        .split('_')
                        .nth(2)?,
                    name,
                ))
            })
            .collect();

        let cantons = zippath
            .get("KT")
            .ok_or("Missing cantons file in archive")?
            .to_string();
        let districts = zippath
            .get("BEZ")
            .ok_or("Missing districts file in archive")?
            .to_string();
        let municipalities = zippath
            .get("GDE")
            .ok_or("Missing municipalities file in archive")?
            .to_string();

        Ok(Self {
            zip,
            cantons,
            districts,
            municipalities,
        })
    }

    fn iter<T>(&mut self, file: String) -> Result<Iter<T>, Box<dyn error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let reader = self.zip.by_name(&file)?;
        let decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(ENCODING))
            .build(reader);
        Ok(CsvReaderBuilder::new()
            .ascii()
            .delimiter(b'\t')
            .terminator(csv::Terminator::CRLF)
            .quoting(false)
            .has_headers(false)
            .from_reader(decoder)
            .into_deserialize())
    }
    /// Iter on all cantons
    pub fn cantons(&mut self) -> Result<Iter<Canton>, Box<dyn error::Error>> {
        self.iter(self.cantons.clone())
    }
    /// Iter on all districts
    pub fn districts(&mut self) -> Result<Iter<District>, Box<dyn error::Error>> {
        self.iter(self.districts.clone())
    }
    /// Iter on all municipalities
    pub fn municipalities(&mut self) -> Result<Iter<Municipality>, Box<dyn error::Error>> {
        self.iter(self.municipalities.clone())
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

/// Status of municipality. All step (municipality, canton and national) are done if status is
/// [Status::Final]
///
/// Dieses Merkmal dient der Unterscheidung von Mutationen, die alle Verfahren auf Stufe Gemeinde,
/// Kanton und Bund durchlaufen haben (1 = definitiv), und denjenigen, die noch nicht alle
/// Verfahren durchlaufen haben (0 = provisorisch).
///
/// Ce caractère sert à différencier les mutations qui sont passées par toutes les étapes, à
/// l’échelon de la commune, du canton et de la Confédération (1 = définitif) de celles qui n’ont
/// pas encore franchi toutes les étapes (0 = provisoire).
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
    /// Formale Neunummerierung Gemeinde/Bezirk / Renumérotation formelle de la commune/du district
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
    /// Formale Neunummerierung Gemeinde/Bezirk / Renumérotation formelle de la commune/du district
    FormalRenumbering = 27,
    /// Aufhebung Gemeinde/Bezirk / Radiation commune/district
    Radiation = 29,
    /// Mutation annulliert / Annulation de la mutation
    MutationAnnulled = 30,
}

/// Canton / Kanton / Canton
#[derive(Debug, Deserialize)]
pub struct Canton {
    id: CantonId,
    abbreviation: String,
    long_name: String,
    #[serde(with = "crate::serde::date_dd_mm_yyyyy_dotted")]
    date_of_change: Date,
}
impl Canton {
    /// Canton number / Kantonsnummer / Numéro du canton
    pub fn id(&self) -> CantonId {
        self.id
    }
    /// Canton's abbreviation / Kantonskürzel / Abréviation du canton
    pub fn abbreviation(&self) -> &str {
        self.abbreviation.as_str()
    }
    /// Canton's name / Kantonsname / Nom du canton
    pub fn name(&self) -> &str {
        self.long_name.as_str()
    }
    /// Change date / Änderungsdatum / Date de modification
    pub fn date_of_change(&self) -> &Date {
        &self.date_of_change
    }
}

/// Bezirk / District
#[derive(Debug, Deserialize)]
pub struct District {
    hist_id: DistrictHistId,
    canton_id: CantonId,
    id: DistrictId,
    long_name: String,
    short_name: String,
    entry_mode: DistrictMode,
    admission_number: MutationId,
    admission_mode: AdmissionMode,
    #[serde(with = "crate::serde::date_dd_mm_yyyyy_dotted")]
    admission_date: Date,
    abolition_number: Option<MutationId>,
    abolition_mode: Option<AbolitionMode>,
    #[serde(with = "crate::serde::option_date_dd_mm_yyyyy_dotted")]
    abolition_date: Option<Date>,
    #[serde(with = "crate::serde::date_dd_mm_yyyyy_dotted")]
    date_of_change: Date,
}
impl District {
    /// Historic identifier
    /// / Historisierungsnummer BEZ
    /// / Numéro d’historisation DIS
    pub fn hist_id(&self) -> DistrictHistId {
        self.hist_id
    }
    /// Canton identifier
    /// / Kantonsnummer
    /// / Numéro du canton
    pub fn canton_id(&self) -> CantonId {
        self.canton_id
    }
    /// District identifier
    /// / Bezirksnummer
    /// / Numéro du district
    pub fn id(&self) -> DistrictId {
        self.id
    }
    /// District name
    /// / Bezirksname
    /// / Nom du district
    pub fn name(&self) -> &str {
        self.long_name.as_str()
    }
    /// District abbreviated name
    /// / Bezirksname kurz
    /// / Nom du district en abrégé
    pub fn short_name(&self) -> &str {
        self.short_name.as_str()
    }
    /// Entry type
    /// / Art des Eintrages
    /// / Type d’entrée
    pub fn entry_mode(&self) -> DistrictMode {
        self.entry_mode
    }
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
    /// Date of the last change
    /// / Änderungsdatum
    /// / Date de modification
    pub fn date_of_change(&self) -> &Date {
        &self.date_of_change
    }
}

#[derive(Debug, Deserialize)]
/// Municipality / Gemeinden / Commune
pub struct Municipality {
    hist_id: MunicipalityHistId,
    district_hist_id: DistrictHistId,
    canton_abbreviation: String,
    id: MunicipalityId,
    long_name: String,
    short_name: String,
    entry_mode: MunicipalityMode,
    status: Status,
    admission_number: MutationId,
    admission_mode: AdmissionMode,
    #[serde(with = "crate::serde::date_dd_mm_yyyyy_dotted")]
    admission_date: Date,
    abolition_number: Option<MutationId>,
    abolition_mode: Option<AbolitionMode>,
    #[serde(with = "crate::serde::option_date_dd_mm_yyyyy_dotted")]
    abolition_date: Option<Date>,
    #[serde(with = "crate::serde::date_dd_mm_yyyyy_dotted")]
    date_of_change: Date,
}
impl Municipality {
    /// Municipality historical identifier
    pub fn hist_id(&self) -> MunicipalityHistId {
        self.hist_id
    }
    /// District historical identifier
    pub fn district_hist_id(&self) -> DistrictHistId {
        self.district_hist_id
    }
    /// Abbreviation of canton (two letter)
    pub fn canton_abbreviation(&self) -> &str {
        self.canton_abbreviation.as_str()
    }
    /// Municipality identifier
    pub fn id(&self) -> MunicipalityId {
        self.id
    }
    /// Municipality official name
    pub fn name(&self) -> &str {
        self.long_name.as_str()
    }
    /// Municipality abbreviated name
    pub fn short_name(&self) -> &str {
        self.short_name.as_str()
    }
    /// Type of municipality
    pub fn entry_mode(&self) -> MunicipalityMode {
        self.entry_mode
    }
    /// Status of change
    pub fn status(&self) -> Status {
        self.status
    }
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
    /// Date of the last change
    /// / Änderungsdatum
    /// / Date de modification
    pub fn date_of_change(&self) -> &Date {
        &self.date_of_change
    }
}

#[derive(Debug)]
/// A mutation data (admission or abolition)
pub struct Mutation<'a, Mode: Copy> {
    number: MutationId,
    mode: Mode,
    date: &'a Date,
}
impl<Mode: Copy> Mutation<'_, Mode> {
    /// Identifier of Mutation
    pub fn id(&self) -> MutationId {
        self.number
    }
    /// Reason of mutation
    pub fn mode(&self) -> Mode {
        self.mode
    }
    /// Date of mutation
    pub fn date(&self) -> &Date {
        self.date
    }
}
