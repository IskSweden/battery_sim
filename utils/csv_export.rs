use std::fs::File;
use std::path::Path;
use anyhow::Result;
use csv::Writer;
use crate::model::srl::SRLEntry;

pub fn save_to_csv(path: &str, entries: &[SRLEntry]) -> Result<()> {
    // TODO:
    // 1. Take vector from srl_importer
    // create a csv (maybe only tmp)
    // Put data into csv
    // cleanup tmp csv?

    let file = File::create(path)?;

    let mut wtr = Writer::from_writer(file);


}