use std::fs::File;
use std::path::Path;
use anyhow::Result;
use csv::Writer;
use crate::model::srl::SRLEntry;
use serde::Serialize;


// function creates a csv file, and serializes pushed vector values from srl.rs in a for loop into the csv file.
pub fn save_to_csv<T: Serialize>(path: &str, entries: &[T]) -> Result<()> {

    let file = File::create(path)?;

    let mut wtr = Writer::from_writer(file);

    for entry in entries {
        wtr.serialize(entry)?
    }
    wtr.flush()?;
    Ok(())
}