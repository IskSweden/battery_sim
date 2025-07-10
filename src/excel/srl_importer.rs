use calamine::{open_workbook_auto, Reader};
use anyhow::{Result};
use crate::model::srl::SRLEntry;
use crate::utils::{parse_number, parse_timestamp_dmy}; // helper

// Load srl data in sheet "Zeitreihen0h15 and throw error if not found. Result of function is a vector in SRLEntry in model/srl.rs"
pub fn load_srl(path: &str) -> Result<Vec<SRLEntry>> {
    let mut workbook = open_workbook_auto(path)?;


    let range = workbook
        .worksheet_range("Zeitreihen0h15")?;

    // new mutatable vector to store entries in
    let mut entries = Vec::new();

    // iterate through rows, skipping first two as they are headers
    for row in range.rows().skip(2) {

        let timestamp = parse_timestamp_dmy(&row[0])?; //Row A

        let pos_energy_kwh = parse_number(&row[6])?; //Row G

        let neg_energy_kwh = parse_number(&row[7])?; //Row H

        let pos_price_eur_mwh = parse_number(&row[21])?; // Row V

        let neg_price_eur_mwh = parse_number(&row[22])?; // Row W

        // Adjust rows to excel file. Given input excel had these rows, so index as needed for these.

        // Pushing entries to vector in srl.rs
        entries.push(SRLEntry {
            timestamp,
            pos_energy_kwh,
            neg_energy_kwh,
            pos_price_eur_mwh,
            neg_price_eur_mwh,
        });
    }

    Ok(entries)
}