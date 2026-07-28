// physics/optics/src/main.rs

//! # Optics Workspace Binary
//!
//! Executable entry point for calculating Planck radiation spectra and generating engineering reports.

use std::path::PathBuf;

/// 1. Main entry point for the optics CLI tool.
fn main() -> core_io::Result<()> {
    // 01. Define target path for the engineering report output
    let output_path = PathBuf::from("target/reports/planck_engineering_report_freq.txt");

    // 02. Specify input parameters for Planck radiation calculation (frequency basis)
    //
    // Example: frequency band analogous to Python example
    // 5.0e13 Hz  = 50 THz
    // 2.0e14 Hz  = 200 THz
    let frequency_range_hz = (5.0e13, 2.0e14);
    let temperature_k = 3500.0; // Example temperature, e.g., black body at 3500 K

    // 03. Execute the Planck engineering report generation using core_io
    optics::planck::create_planck_engineering_report(
        &output_path,
        frequency_range_hz,
        temperature_k,
    )?;

    println!("Engineering report successfully generated at: {:?}", output_path);

    Ok(())
}