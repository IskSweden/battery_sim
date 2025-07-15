use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub capacity_kwh: f64,
    pub c_rate: f64,
    pub efficiency: f64,           // 0.95 → 95%
    pub min_soc_frac: f64,         // e.g. 0.1
    pub initial_soc_frac: f64,     // e.g. 0.5
    pub reserve_fraction: f64,     // e.g. 0.3 = 30% reserved for SRL
    pub transformer_limit_kw: f64, // e.g. 240.0
    pub timestep_minutes: f64,     // usually 1.0
    pub battery_price_per_kwh_chf: f64,
    pub operating_cost_rate: f64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            capacity_kwh: 1000.0,
            c_rate: 1.0,
            efficiency: 0.95,
            min_soc_frac: 0.1,
            initial_soc_frac: 0.5,
            reserve_fraction: 0.5,
            transformer_limit_kw: 240.0,
            timestep_minutes: 1.0,
            battery_price_per_kwh_chf: 400.0,
            operating_cost_rate: 0.01,
        }
    }
}

#[derive(Debug)]
pub struct SimulationSummary {
    pub total_ticks: usize,

    // Energy flows
    pub total_srl_out_kwh: f64,
    pub total_srl_in_kwh: f64,
    pub total_ps_out_kwh: f64,
    pub total_ps_in_kwh: f64,

    // SoC extremes
    pub min_soc_kwh: f64,
    pub max_soc_kwh: f64,

    // transformer
    pub transformer_violations: usize,

    // Economics -> to be expanded
    pub total_srl_revenue_chf: f64,
    pub peak_shaving_savings_chf: f64,
    pub amortization_years: Option<f64>,

    // Battery wear / Cycles
    pub battery_cycles: f64,
}
