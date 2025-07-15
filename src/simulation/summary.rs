use super::config::{SimulationConfig, SimulationSummary};
use super::tick_result::SimulationTickResult;
use chrono::Datelike;
use std::collections::HashMap;

pub fn summarize(ticks: &[SimulationTickResult], config: &SimulationConfig) -> SimulationSummary {
    let mut summary = SimulationSummary {
        total_ticks: ticks.len(),

        total_srl_out_kwh: 0.0,
        total_srl_in_kwh: 0.0,
        total_ps_out_kwh: 0.0,
        total_ps_in_kwh: 0.0,

        min_soc_kwh: f64::MAX,
        max_soc_kwh: f64::MIN,

        transformer_violations: 0,

        total_srl_revenue_chf: 0.0,
        peak_shaving_savings_chf: 0.0,
        battery_cycles: 0.0,

        amortization_years: None,
    };

    let mut monthly_peak_before: HashMap<(i32, u32), f64> = HashMap::new();
    let mut monthly_peak_after: HashMap<(i32, u32), f64> = HashMap::new();

    for tick in ticks {
        summary.total_srl_out_kwh += tick.srl_energy_out_kwh;
        summary.total_srl_in_kwh += tick.srl_energy_in_kwh;

        summary.total_ps_out_kwh += tick.battery_out_kw * (1.0 / 60.0);
        summary.total_ps_in_kwh += tick.battery_in_kw * (1.0 / 60.0);

        summary.min_soc_kwh = summary.min_soc_kwh.min(tick.soc_kwh);
        summary.max_soc_kwh = summary.max_soc_kwh.max(tick.soc_kwh);

        if tick.transformer_violation {
            summary.transformer_violations += 1;
        }

        summary.total_srl_revenue_chf += tick.srl_revenue_pos_chf;
        summary.total_srl_revenue_chf += tick.srl_revenue_neg_chf;

        let year = tick.timestamp.year();
        let month = tick.timestamp.month();
        let key = (year, month);

        monthly_peak_before
            .entry(key)
            .and_modify(|v| *v = v.max(tick.original_grid_kw))
            .or_insert(tick.original_grid_kw);

        monthly_peak_after
            .entry(key)
            .and_modify(|v| *v = v.max(tick.final_grid_kw))
            .or_insert(tick.final_grid_kw);
    }

    let mut total_peak_saving_chf = 0.0;
    let tariff = 10.0; // CHF per kW per month

    for key in monthly_peak_before.keys() {
        let before = monthly_peak_before.get(key).unwrap_or(&0.0);
        let after = monthly_peak_after.get(key).unwrap_or(&0.0);

        let saved_kw = (before - after).max(0.0);
        let saved_chf = saved_kw * tariff;
        total_peak_saving_chf += saved_chf;
    }

    let usable_capacity = summary.max_soc_kwh - summary.min_soc_kwh;

    if usable_capacity > 0.0 {
        let total_throughput_kwh = summary.total_ps_out_kwh
            + summary.total_ps_in_kwh
            + summary.total_srl_out_kwh
            + summary.total_srl_in_kwh;

        summary.battery_cycles = total_throughput_kwh / (2.0 * usable_capacity);
    } else {
        summary.battery_cycles = 0.0;
    }

    summary.peak_shaving_savings_chf = total_peak_saving_chf;

    // Ammortization
    let capacity = config.capacity_kwh;
    let price_per_kwh = config.battery_price_per_kwh_chf;
    let invest = capacity * price_per_kwh;

    let op_cost = invest * config.operating_cost_rate;
    let total_revenue = summary.total_srl_revenue_chf + summary.peak_shaving_savings_chf;

    if total_revenue > 0.0 {
        summary.amortization_years = Some((invest + op_cost) / total_revenue);
    } else {
        summary.amortization_years = None;
    }

    summary
}

impl SimulationSummary {
    pub fn print(&self) {
        println!("\n===== Simulation Summary =====\n");

        println!("Total ticks:              {}", self.total_ticks);
        println!("-------------------------------");
        println!(
            "SRL delivered (discharge): {:>8.2} kWh",
            self.total_srl_out_kwh
        );
        println!(
            "SRL absorbed  (charge):    {:>8.2} kWh",
            self.total_srl_in_kwh
        );
        println!(
            "PS discharge (out):        {:>8.2} kWh",
            self.total_ps_out_kwh
        );
        println!(
            "PS charge    (in):         {:>8.2} kWh",
            self.total_ps_in_kwh
        );
        println!("-------------------------------");
        println!(
            "Min SoC: {:>6.1} kWh     Max SoC: {:>6.1} kWh",
            self.min_soc_kwh, self.max_soc_kwh
        );
        println!("Transformer violations:    {}", self.transformer_violations);
        println!("===============================\n");
        println!("Economics");
        println!("-------------------------------\n");
        println!(
            "SRL revenue total:            {:>8.2} CHF",
            self.total_srl_revenue_chf
        );
        println!(
            "Peak shaving savings:          {:>8.2} CHF",
            self.peak_shaving_savings_chf
        );

        println!(
            "Total revenue:                {:>8.2} CHF",
            self.total_srl_revenue_chf + self.peak_shaving_savings_chf
        );

        match self.amortization_years {
            Some(years) => println!("Estimated payback time:    {:>6.1} years", years),
            None => println!("Estimated payback time            Not achievable"),
        }

        println!("================================\n");
        println!("Estimated battery cycles:   {:>8.2}", self.battery_cycles);

        println!("===============================\n");
    }
}
