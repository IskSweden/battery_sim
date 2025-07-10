use std::fs::File;
use std::path::Path;
use anyhow::Result;
use csv::Writer;
use crate::model::srl::SRLEntry;


// function creates a csv file, and serializes pushed vector values from srl.rs in a for loop into the csv file.
pub fn save_to_csv(path: &str, entries: &[SRLEntry]) -> Result<()> {
    // TODO:
    // 1. Take vector from srl_importer
    // create a csv (maybe only tmp)
    // Put data into csv
    // cleanup tmp csv?

    let file = File::create(path)?;

    let mut wtr = Writer::from_writer(file);

    for entry in entries {
        wtr.serialize(entry)?
    }
    wtr.flush()?;
    Ok(())
}