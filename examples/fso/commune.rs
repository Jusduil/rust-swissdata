use std::collections::{BTreeSet, HashMap, HashSet};
use std::env;
use std::error;

use reqwest;
use swissdata::fso::communes::*;
use swissdata::tools::dataset::Datastore;

fn main() -> Result<(), Box<dyn error::Error>> {
    let canton_abr = env::args().nth(1).unwrap_or("be".into()).to_uppercase();

    let store = datastore();
    let ds = store.load(&reqwest::blocking::Client::new())?;
    let meta = store.meta();
    println!(
        "Data editor        : {}",
        meta.editor.get_or_default("fr").name()
    );
    println!(
        "Copyright          : {} -> {}",
        meta.copyright.get_or_default("fr").name(),
        meta.copyright.get_or_default("fr").url()
    );
    println!("Terms of use       : {:?}", meta.terms.get_or_default("fr"));
    let terms = meta.terms_automatic;
    println!("Commercial use     : {:?}", terms.free_commercial_use);
    println!("Non-commercial use : {:?}", terms.free_noncommercial_use);
    println!("Citation required  : {:?}", terms.citation_mandatory);

    let kt = ds
        .cantons
        .iter()
        .filter_map(Result::ok)
        .find(|kt| kt.abbreviation == canton_abr)
        .expect("Missing Bern canton in data");

    let mut districts: Vec<_> = ds
        .districts
        .actual()
        .filter_map(Result::ok)
        .filter(|d| &d.canton_id == &kt.id)
        .collect();
    districts.sort_by_key(|d| (d.entry_mode, d.short_name.clone()));

    println!("{} has {} districts", kt.name, districts.len());
    println!(
        "{} and {} districts in historic",
        kt.name.replace(|_| true, " "),
        ds.districts
            .historic()
            .filter_map(Result::ok)
            .filter(|d| &d.canton_id == &kt.id)
            .count()
    );

    for d in &districts {
        let mut mun: Vec<_> = ds
            .municipalities
            .iter()
            .filter_map(Result::ok)
            .filter(|m| &m.district_hist_id == &d.hist_id)
            .filter(|m| m.abolition().is_none())
            .collect();
        println!(
            "{:<18} {} has {} municipalities/areas",
            format!("({:?})", d.entry_mode),
            d.short_name,
            mun.len()
        );

        let types: BTreeSet<_> = mun.iter().map(|m| m.entry_mode).collect();
        mun.sort_by_key(|m| m.name.clone());
        for t in types {
            let names: Vec<_> = mun
                .iter()
                .filter(|m| m.entry_mode == t)
                .map(|m| m.name.as_str())
                .collect();
            let mut chunks = names.chunks(10);
            if let Some(c) = chunks.next() {
                println!("                     {:?}: {}", t, c.join(", "));
            }
            for c in chunks {
                println!(
                    "                     {}  {}",
                    format!("{:?}", t).replace(|_| true, " "),
                    c.join(", ")
                );
            }
        }
    }

    Ok(())
}
