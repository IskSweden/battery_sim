use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MergedTick {
    pub timestamp: DateTime<Utc>,
    pub power_kw: f64,
    pub srl_pos_kwh: f64,
    pub srl_neg_kwh: f64,
    pub srl_pos_price_eur_mwh: f64,
    pub srl_neg_price_eur_mwh: f64,
}
