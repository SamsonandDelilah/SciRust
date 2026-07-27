// physics/optics/src/main.rs

//! # Optics Workspace Binary
//! 
//! Executable entry point for calculating Planck radiation spectra and generating engineering reports.

use std::path::PathBuf;

/// 1. Main entry point for the optics CLI tool.
fn main() -> core_io::Result<()> {
    // 01. Define target path for the engineering report output
    let output_path = PathBuf::from("target/reports/planck_engineering_report.txt");

    // 02. Specify input parameters for Planck radiation calculation
    let wavelength_range_nm = (280.0, 900.0);
    let temperature_k = 5778.0; // Effective temperature of the Sun

    // 03. Execute the Planck engineering report generation using core_io
    optics::planck::create_planck_engineering_report(
        &output_path,
        wavelength_range_nm,
        temperature_k,
    )?;

    println!("Engineering report successfully generated at: {:?}", output_path);

    Ok(())
}