#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
//#![feature(rustdoc_missing_doc_code_examples)]
//#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::private_doc_tests)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]
//#![deny(warnings)]
#![feature(type_changing_struct_update)]

//! Crate for access some data of Switzerland
//!
//! - [OpenData Swiss](https://opendata.swiss/)
//! - [eCH norm](https://www.ech.ch/fr)
//!
//!
//! TODO:
//! - <https://www.bfs.admin.ch/bfs/fr/home/bases-statistiques/repertoire-officiel-communes-suisse/tableau-correspondance-regbl.html>
//! - <https://www.cadastre.ch/fr/services/service/registry/street.html>
//! - <https://www.cadastre.ch/fr/av.html>

pub mod fso;
pub mod tools;

// internal tools
use tools::internal::serde as i_serde;

// TODO
// https://www.cadastre.ch/fr/av.html

pub use chrono::NaiveDate as Date;
