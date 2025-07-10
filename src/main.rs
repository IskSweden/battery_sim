mod excel;
mod model;
mod utils;

use anyhow::Result;
use excel::srl_importer::load_srl;
use excel::load_importer::load_load_curve;
use utils::csv_export::save_to_csv;


fn main() -> Result<()> {
    println!("Loading SRL Excel...");
    let srl_entries = load_srl("data/input/input_srl.xlsx")?;
    println!("Loaded {} SRL entries", srl_entries.len());
    save_to_csv("data/output/srl_cleaned.csv", &srl_entries)?;
    println!("Saved srl_cleaned.csv");

    println!("Loading Load Curve Excel...");
    let load_entries = load_load_curve("data/input/input_wirkleistung.xlsx")?;
    println!("Loaded {} load curve entries", load_entries.len());
    save_to_csv("data/output/load_cleaned.csv", &load_entries)?;
    println!("Saved load_cleaned.csv");

    Ok(())
}   
