use super::config::SimulationSummary;
use super::tick_result::SimulationTickResult;
use chrono::Datelike;
use std::collections::HashMap;

pub fn summarize(ticks: &[SimulationTickResult]) -> SimulationSummary {
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

    summary.peak_shaving_savings_chf = total_peak_saving_chf;

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
        println!(
            "SRL revenue total:         {:>8.2} CHF",
            self.total_srl_revenue_chf
        );
        println!(
            "Peak shaving savings:   {:>8.2} CHF",
            self.peak_shaving_savings_chf
        );

        println!("===============================\n");
    }
}
