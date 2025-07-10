use calamine::{open_workbook_auto, Reader};  // For reading Excel
use anyhow::{Result};              // For error handling
use crate::model::timeseries::LoadEntry;   // output struct
use crate::utils::{parse_number, parse_timestamp_ymd}; // helper 

pub fn load_load_curve(path: &str) -> Result<Vec<LoadEntry>> {

    let mut workbook = open_workbook_auto(path)?;

    let range = workbook.worksheet_range("Lastgang")?;

    let mut entries = Vec::new();

    for row in range.rows().skip(1) {

        let timestamp = parse_timestamp_ymd(&row[0])?; //Row A

        let power_kw = parse_number(&row[1])?; //Row B

        entries.push(LoadEntry {
        timestamp,
        power_kw,

        });
    }
    Ok(entries)

}