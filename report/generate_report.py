#!/bin/bash

echo "Running Rust simulation..."
cargo run --release

echo "Generating plots with Python..."
source venv/bin/activate
python3 report/generate_report.py

echo "Done. Plots saved to data/output/"
