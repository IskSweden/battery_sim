// Project modules
mod excel;
mod model;
mod simulation;
mod utils;

use std::ptr::slice_from_raw_parts_mut;

// Error handling
use anyhow::Result;

// CSV loading from Excel
use excel::load_importer::load_load_curve;
use excel::srl_importer::load_srl;

// Interpolation tools
use utils::interpolation::{generate_time_grid, interpolate_load_to_1min, interpolate_srl_to_1min};

// CSV export
use utils::csv_export::save_to_csv;
use utils::file_exists;
use utils::merging_csv::merge_1min_series;

// Models
use model::mergedseries::MergedTick;
use model::srl::SRLEntry;
use model::timeseries::LoadEntry;

// simulation
use simulation::config::SimulationConfig;
use simulation::config::SimulationSummary;
use simulation::engine::run_simulation;
use simulation::summary::summarize;

// External
use chrono::{DateTime, Utc};
use csv::Reader;

fn main() -> Result<()> {
    let merged_path = "data/output/merged_timeseries.csv";
    let merged_entries: Vec<MergedTick>;

    if file_exists(merged_path) {
        println!("Found existing merged_timeseries.csv — skipping import/interpolation.");
        let mut rdr = csv::Reader::from_path(merged_path)?;
        merged_entries = rdr.deserialize().collect::<Result<_, _>>()?;
    } else {
        println!("Merged CSV not found. Running full pipeline...");

        let srl_entries = load_srl("data/input/input_srl.xlsx")?;
        let load_entries = load_load_curve("data/input/input_wirkleistung.xlsx")?;

        let (start, end) = (
            load_entries.first().unwrap().timestamp,
            load_entries.last().unwrap().timestamp,
        );

        let time_grid = generate_time_grid(start, end, 1);
        let load_1min = interpolate_load_to_1min(&load_entries, &time_grid);
        let srl_1min = interpolate_srl_to_1min(&srl_entries, &time_grid);
        merged_entries = merge_1min_series(&load_1min, &srl_1min);

        save_to_csv("data/output/load_cleaned.csv", &load_1min)?;
        save_to_csv("data/output/srl_cleaned.csv", &srl_1min)?;
        save_to_csv(merged_path, &merged_entries)?;
        println!(
            "Data pipeline finished. {} Entries ready.",
            merged_entries.len()
        )
    }
    println!("Starting simulation");
    let mut config = SimulationConfig::default();

    config.initial_soc_frac = 0.6;
    config.reserve_fraction = 0.2;

    let sim_results = run_simulation(&merged_entries, &config);

    save_to_csv("data/output/simulation.results.csv", &sim_results)?;
    println!("Exported to simulation.results.csv");

    let summary = summarize(&sim_results, &config);
    summary.print();

    println!("Simulation complete. Total ticks: {}", sim_results.len());
    Ok(())
}
