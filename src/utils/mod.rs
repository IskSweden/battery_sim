pub mod csv_export;
pub mod interpolation;
pub mod merging_csv;


use chrono::{NaiveDateTime, DateTime, Utc};
use calamine::DataType;
use anyhow::{Result, anyhow};

use std::path::Path;

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()

}

// parses dmy timestamps from dym to correct float values
pub fn parse_timestamp_dmy(cell: &DataType) -> Result<DateTime<Utc>> {
    match cell {
        DataType::String(s) => {
            // Clean up newline or carriage return issues
            let cleaned = s.replace('\n', " ").replace('\r', "").trim().to_string();
            NaiveDateTime::parse_from_str(&cleaned, "%d.%m.%Y %H:%M")
                .map(|naive| DateTime::from_naive_utc_and_offset(naive, Utc))
                .map_err(|e| anyhow!("Invalid DMY timestamp: {}", e))
        }
        DataType::Float(f) => {
            let base = NaiveDateTime::parse_from_str("1899-12-30 00:00", "%Y-%m-%d %H:%M")?;
            let naive = base + chrono::Duration::milliseconds((*f * 86400.0 * 1000.0) as i64);
            Ok(DateTime::from_naive_utc_and_offset(naive, Utc))
        }
        _ => Err(anyhow!("Invalid DMY timestamp format")),
    }
}


// Parses ymd format timestamp from excel datatype to float.
pub fn parse_timestamp_ymd(cell: &DataType) -> Result<DateTime<Utc>> {
    match cell {
        DataType::String(s) => {
            let cleaned = s.replace('\n', " ").replace('\r', "").trim().to_string();
            NaiveDateTime::parse_from_str(&cleaned, "%Y-%m-%d %H:%M")
                .map(|naive| DateTime::from_naive_utc_and_offset(naive, Utc))
                .map_err(|e| anyhow!("Invalid YMD timestamp: {}", e))
        }
        DataType::Float(f) | DataType::DateTime(f) => {
            let base = NaiveDateTime::parse_from_str("1899-12-30 00:00", "%Y-%m-%d %H:%M")?;
            let naive = base + chrono::Duration::milliseconds((*f * 86400.0 * 1000.0) as i64);
            Ok(DateTime::from_naive_utc_and_offset(naive, Utc))

        }
        _ => Err(anyhow!("Invalid YMD timestamp format")),
    }
}



// parse numbers and values to correct format. excplicitly clean "," to ".".
pub fn parse_number(cell: &DataType) -> Result<f64> {
    match cell {
        DataType::Float(f) => Ok(*f),
        DataType::String(s) => {
            let cleaned = s.replace(",", ".").trim().to_string();
            cleaned.parse::<f64>().map_err(|e| anyhow!("Invalid float: {}", e))
        }
        _ => Err(anyhow!("Invalid number format")),
    }
}
