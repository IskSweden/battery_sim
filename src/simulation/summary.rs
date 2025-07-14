use super::config::SimulationSummary;
use super::tick_result::SimulationTickResult;

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
    }
}
