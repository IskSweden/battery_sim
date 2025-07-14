use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SimulationTickResult {
    pub timestamp: DateTime<Utc>,

    // Inputs
    pub original_power_kw: f64,
    pub srl_pos_kwh: f64,
    pub srl_neg_kwh: f64,

    // Battery behavior
    pub battery_in_kw: f64,  // charging
    pub battery_out_kw: f64, // discharging

    // SRL response
    pub srl_energy_in_kwh: f64,  // energy absorbed
    pub srl_energy_out_kwh: f64, // energy delivered

    // State of Charge
    pub soc_kwh: f64,
    pub soc_percent: f64,

    // Net power at grid (after all battery + srl)
    pub grid_net_kw: f64,

    // Limit flag
    pub transformer_violation: bool,

    // SRL Revenue
    pub srl_revenue_pos_chf: f64,
    pub srl_revenue_neg_chf: f64,

    pub original_grid_kw: f64,
    pub final_grid_kw: f64,
}
