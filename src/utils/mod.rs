pub mod csv_export;
pub mod datetime;


use chrono::NaiveDateTime;
use calamine::DataType;
use anyhow::{Result, anyhow};

pub fn parse_timestamp_dmy(cell: &DataType) -> Result<NaiveDateTime> {
    match cell {
        DataType::String(s) => {
            // Clean up newline or carriage return issues
            let cleaned = s.replace('\n', " ").replace('\r', "").trim().to_string();
            NaiveDateTime::parse_from_str(&cleaned, "%d.%m.%Y %H:%M")
                .map_err(|e| anyhow!("Invalid DMY timestamp: {}", e))
        }
        DataType::Float(f) => {
            let base = NaiveDateTime::parse_from_str("1899-12-30 00:00", "%Y-%m-%d %H:%M")?;
            Ok(base + chrono::Duration::milliseconds((*f * 86400.0 * 1000.0) as i64))
        }
        _ => Err(anyhow!("Invalid DMY timestamp format")),
    }
}

pub fn parse_timestamp_ymd(cell: &DataType) -> Result<NaiveDateTime> {
    match cell {
        DataType::String(s) => {
            let cleaned = s.replace('\n', " ").replace('\r', "").trim().to_string();
            NaiveDateTime::parse_from_str(&cleaned, "%Y-%m-%d %H:%M")
                .map_err(|e| anyhow!("Invalid YMD timestamp: {}", e))
        }
        DataType::Float(f) | DataType::DateTime(f) => {
            let base = NaiveDateTime::parse_from_str("1899-12-30 00:00", "%Y-%m-%d %H:%M")?;
            Ok(base + chrono::Duration::milliseconds((*f * 86400.0 * 1000.0) as i64))
        }
        _ => Err(anyhow!("Invalid YMD timestamp format")),
    }
}


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
