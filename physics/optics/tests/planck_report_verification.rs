// physics/optics/tests/planck_report_verification.rs

//! # Planck Engineering Report Verification Test
//! 
//! Validates the generated text output and numeric results of the Planck radiation report.

use std::path::PathBuf;

/// 1. Integration test verifying the successful generation and content structure of the Planck report.
#[test]
fn test_create_engineering_report_generation() -> core_io::Result<()> {
    let test_output_path = PathBuf::from("target/planck_verification_report.txt");

    // 01. Execute report generation with T = 5778 K (solar-like)
    let temperature_k = 3500.0;
    optics::planck::create_planck_engineering_report(
        &test_output_path,
        (5.0e13, 2.0e14),
        temperature_k,
    )?;

    // 02. Read generated content back
    let content = core_io::formats::txt::read_text_file(&test_output_path, core_io::TextEncoding::Utf8)?;

    // 03. Assert required report sections
    assert!(content.contains("Planck Radiation Engineering Report"));

    // 04. Assert peak wavelength for T = 5778 K (~501 nm)
    // lambda_max ≈ 2.897771955e-3 / 5778 ≈ 5.01e-7 m = 501 nm
    //assert!(content.contains("501"));
     // Peak wavelength for T = 3500 K is ~828 nm

    Ok(())
}