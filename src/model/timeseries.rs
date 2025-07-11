use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoadEntry {
    pub timestamp: DateTime<Utc>,
    pub power_kw: f64,
}
