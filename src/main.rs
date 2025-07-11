// Project modules
mod excel;
mod model;
mod utils;

// Error handling
use anyhow::Result;

// CSV loading from Excel
use excel::load_importer::load_load_curve;
use excel::srl_importer::load_srl;

// Interpolation tools
use utils::interpolation::{
    generate_time_grid,
    interpolate_load_to_1min,
    interpolate_srl_to_1min,
};

// CSV export
use utils::csv_export::save_to_csv;

// Models / Structs
use model::timeseries::LoadEntry;
use model::srl::SRLEntry;
use model::mergedseries::MergedTick;
use utils::merging_csv::merge_1min_series;

// Date handling
use chrono::{DateTime, Utc};

fn main() -> Result<()> {
    // Load raw Excel inputs
    println!("Loading SRL Excel...");
    let srl_entries: Vec<SRLEntry> = load_srl("data/input/input_srl.xlsx")?;
    println!("Loaded {} SRL entries", srl_entries.len());

    println!("Loading Load Curve Excel...");
    let load_entries: Vec<LoadEntry> = load_load_curve("data/input/input_wirkleistung.xlsx")?;
    println!("Loaded {} Load entries", load_entries.len());


    let start = load_entries.first().unwrap().timestamp;
    let end = load_entries.last().unwrap().timestamp;



    let time_grid = generate_time_grid(start, end, 1);

    let load_1min = interpolate_load_to_1min(&load_entries, &time_grid);

    let srl_1min = interpolate_srl_to_1min(&srl_entries, &time_grid);

    let merged_1min = merge_1min_series(&load_1min, &srl_1min);





    // Save final merged result to simulation-ready CSV
    save_to_csv("data/output/load_cleaned.csv", &load_1min)?;
    save_to_csv("data/output/srl_cleaned.csv", &srl_1min)?;
    save_to_csv("data/output/merged_timeseries.csv", &merged_1min)?;


    Ok(())
}
