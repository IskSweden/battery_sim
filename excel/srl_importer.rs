use chrono::NaiveDateTime;
use calamine::{open_workbook_auto, Reader, DataType};
use anyhow::{Result, anyhow};
use crate::model::srl::SRLEntry;

// Parse timestamps to native rust understandable stamps. Result of function is a formatted timestamp.
// This function also handles turning the timestamps from strings to floats. retunrs a 64-bit integer, which is easier to work with.
fn parse_timestamp(cell: &DataType) -> Result<NaiveDateTime> {
    match cell {
        DataType::String(s) => {
            NaiveDateTime::parse_from_str(s, "%d.%m.%Y %H:%M").map_err(|e| anyhow!(e))
        }
        DataType::Float(f) => {
            let base = NaiveDateTime::parse_from_str("1899-12-30 00:00", "%Y-%m-%d %H:%M")?;
            Ok(base + chrono::Duration::milliseconds((*f * 86400.0 * 1000.0) as i64))
        }
        _ => Err(anyhow!("Invalid timestamp format")),
    }
}

// Parse numbers and explicitly convert , to . Result of function is a 64-bit float.
fn parse_number(cell: &DataType) -> Result<f64> {
    match cell {
        DataType::Float(f) => Ok(*f),
        DataType::String(s) => {
            let cleaned = s.replace(",", ".").trim().to_string();
            cleaned.parse::<f64>().map_err(|e| anyhow!(e))
        }
        _ => Err(anyhow!("Invalid number format")),
    }
}

// Load srl data in sheet "Zeitreihen0h15 and throw error if not found. Result of function is a vector in SRLEntry in model/srl.rs"
pub fn load_srl(path: &str) -> Result<Vec<SRLEntry>> {
    let mut workbook = open_workbook_auto(path)?;
    let range = workbook
        .worksheet_range("Zeitreihen0h15")
        .ok_or_else(|| anyhow!("Sheet 'Zeitreihen0h15' not found"))??;

    // new mutatable vector to store entries in
    let mut entries = Vec::new();

    // iterate through rows, skipping first two as they are headers
        for row in range.rows().skip(2) {

        let timestamp = parse_timestamp(&row[0])?;

        let pos_energy_kwh = parse_number(&row[1])?;

        let neg_energy_kwh = parse_number(&row[2])?;

        let pos_price_eur_mwh = parse_number(&row[3])?;

        let neg_price_eur_mwh = parse_number(&row[4])?;


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