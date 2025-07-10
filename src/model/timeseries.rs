use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoadEntry {
    pub timestamp: NaiveDateTime,
    pub power_kw: f64,
}
