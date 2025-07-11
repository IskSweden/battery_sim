use crate::model::mergedseries::MergedTick;
use super::tick_result::SimulationTickResult;
use super::config::SimulationConfig;

pub fn run_simulation(
    ticks: &[MergedTick],
    config: &SimulationConfig,
) -> Vec<SimulationTickResult> {
    let mut results = Vec::with_capacity(ticks.len());

    // === Init battery state ===
    let soc_min = config.capacity_kwh * config.min_soc_frac;
    let soc_max = config.capacity_kwh;
    let soc_reserve = (soc_max - soc_min) * config.reserve_fraction;
    let mut soc_kwh = config.capacity_kwh * config.initial_soc_frac;

    // === Constants ===
    let timestep_h = config.timestep_minutes / 60.0;
    let p_max = config.capacity_kwh * config.c_rate;

    for tick in ticks {
        // For now, placeholder values:
        let result = SimulationTickResult {
            timestamp: tick.timestamp,
            original_power_kw: tick.power_kw,
            srl_pos_kwh: tick.srl_pos_kwh,
            srl_neg_kwh: tick.srl_neg_kwh,

            battery_in_kw: 0.0,
            battery_out_kw: 0.0,

            srl_energy_in_kwh: 0.0,
            srl_energy_out_kwh: 0.0,

            soc_kwh,
            soc_percent: 100.0 * (soc_kwh - soc_min) / (soc_max - soc_min),

            grid_net_kw: tick.power_kw,
            transformer_violation: false,
        };

        results.push(result);
    }

    results
}
