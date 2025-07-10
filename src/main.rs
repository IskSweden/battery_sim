mod excel;
mod model;
mod utils;
mod orchestrator;

use anyhow::Result;
use excel::srl_importer::load_srl;
use utils::csv_export::save_to_csv;

fn main() -> Result<()> {
    let entries = load_srl("data/input/input_srl.xlsx")?;
    save_to_csv("data/output/srl_cleaned.csv", &entries)?;
    Ok(())
}
