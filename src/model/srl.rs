use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SRLEntry {
    pub timestamp: DateTime<Utc>,
    pub pos_energy_kwh: f64,
    pub neg_energy_kwh: f64,
    pub pos_price_eur_mwh: f64,
    pub neg_price_eur_mwh: f64,
}
