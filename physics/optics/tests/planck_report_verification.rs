// physics/optics/tests/planck_report_verification.rs

//! # Planck Engineering Report Verification Test
//! 
//! Validates the generated text output and numeric results of the Planck radiation report.

use std::path::PathBuf;

/// 1. Integration test verifying the successful generation and content structure of the Planck report.
#[test]
fn test_planck_engineering_report_generation() -> core_io::Result<()> {
    let test_output_path = PathBuf::from("target/test_reports/planck_verification_report.txt");
    
    // 01. Execute report generation with solar black body parameters
    optics::planck::create_planck_engineering_report(
        &test_output_path,
        (300.0, 800.0),
        5778.0,
    )?;

    // 02. Read generated content back using core_io text module
    let content = core_io::formats::txt::read_text_file(&test_output_path, core_io::TextEncoding::Utf8)?;

    // 03. Assert required report sections and evaluated peak emissions
    assert!(content.contains("Planck Radiation Engineering Report"));
    assert!(content.contains("501")); // Expected peak wavelength ~501.68 nm for 5778 K

    Ok(())
}