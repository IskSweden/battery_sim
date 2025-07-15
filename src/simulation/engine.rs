use super::config::SimulationConfig;
use super::tick_result::SimulationTickResult;
use crate::model::mergedseries::MergedTick;

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
    let eff = config.efficiency;

    // Debug counters
    let mut srl_pos_count = 0;
    let mut srl_neg_count = 0;

    for tick in ticks {
        // SRL response
        let mut srl_energy_in_kwh = 0.0;
        let mut srl_energy_out_kwh = 0.0;

        // max available discharge / charge
        let max_discharge_kwh = ((soc_kwh - soc_min - soc_reserve).max(0.0)) * eff;
        let max_charge_kwh = ((soc_max - soc_kwh - soc_reserve).max(0.0)) / eff;
        let e_max = p_max * timestep_h;

        // Discharge for SRL_pos
        if tick.srl_pos_kwh > 0.0 {
            srl_pos_count += 1;

            let requested = tick.srl_pos_kwh;
            let possible = max_discharge_kwh.min(e_max);
            let fulfilled = requested.min(possible);

            srl_energy_out_kwh = fulfilled;
            soc_kwh -= fulfilled / eff;
        }
        // Charge for SRL_neg
        else if tick.srl_neg_kwh < 0.0 {
            // charge to fulfill SRL_neg
            srl_neg_count += 1;

            let requested = -tick.srl_neg_kwh;
            let possible = max_charge_kwh.min(e_max);
            let fulfilled = requested.min(possible);

            srl_energy_in_kwh = fulfilled;
            soc_kwh += fulfilled * eff;
        }

        // Revenue from SRL
        let revenue_pos = tick.srl_pos_price_eur_mwh / 1000.0 * srl_energy_out_kwh;
        let revenue_neg = -tick.srl_neg_price_eur_mwh / 1000.0 * srl_energy_in_kwh;

        // peak shaving
        let mut battery_in_kw = 0.0;
        let mut battery_out_kw = 0.0;

        if tick.power_kw > 0.0 {
            let requested_kwh = tick.power_kw.min(p_max) * timestep_h;

            let fullfilled_kwh = requested_kwh.min(max_discharge_kwh);

            battery_out_kw = fullfilled_kwh / timestep_h;
            soc_kwh -= fullfilled_kwh;
        } else if tick.power_kw < 0.0 {
            let requested_kwh = (-tick.power_kw).min(p_max) * timestep_h;
            let fulfilled_kwh = requested_kwh.min(max_charge_kwh);

            battery_in_kw = fulfilled_kwh / timestep_h;
            soc_kwh += fulfilled_kwh;
        }

        // SoC and Output
        soc_kwh = soc_kwh.clamp(soc_min, soc_max);
        let soc_percent = 100.0 * (soc_kwh - soc_min) / (soc_max - soc_min);

        let grid_net_kw = tick.power_kw + battery_in_kw - battery_out_kw;
        let transformer_violation = grid_net_kw.abs() > config.transformer_limit_kw;

        // Output Result
        let result = SimulationTickResult {
            timestamp: tick.timestamp,

            original_power_kw: tick.power_kw,
            srl_pos_kwh: tick.srl_pos_kwh,
            srl_neg_kwh: tick.srl_neg_kwh,

            battery_in_kw,
            battery_out_kw,

            srl_energy_in_kwh,
            srl_energy_out_kwh,

            soc_kwh,
            soc_percent,

            original_grid_kw: tick.power_kw,
            final_grid_kw: grid_net_kw,

            grid_net_kw,
            transformer_violation,

            srl_revenue_pos_chf: revenue_pos,
            srl_revenue_neg_chf: revenue_neg,
        };

        results.push(result);
    }

    println!("SRL pos ticks: {}", srl_pos_count);
    println!("SRL neg ticks: {}", srl_neg_count);

    results
}
