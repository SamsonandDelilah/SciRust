// physics/optics/src/planck.rs

//! # Planck's Law & Radiation Module
//! 
//! Computes black body radiation characteristics, evaluates emission maximums via Wien's displacement law, 
//! and integrates spectral radiance using the workspace `integral` library.

use std::path::Path;

/// 1. Calculates the integrated spectral radiance of a black body between two wavelengths
/// using the workspace Gauss-Kronrod 15-point quadrature rule.
/// 
/// # Arguments
/// * `wavelength_start_m` - Start wavelength in meters ($\lambda_1$)
/// * `wavelength_end_m` - End wavelength in meters ($\lambda_2$)
/// * `temperature_k` - Absolute temperature in Kelvin ($T$)
/// 
/// # Returns
/// * Tuple of `(integrated_radiance, estimated_error)` in $\text{W} \cdot \text{m}^{-2} \cdot \text{sr}^{-1}$
pub fn planck_radiance_integral_gk15(
    wavelength_start_m: f64,
    wavelength_end_m: f64,
    temperature_k: f64,
) -> (f64, f64) {
    // 01. Define the closure wrapper for Planck's law function compatible with the integral library
    let integrand = |lam: f64| planck_spectral_radiance(lam, temperature_k);

    // 02. Invoke the Gauss-Kronrod 15-point rule directly from the workspace integral crate
    integral::methods::gauss_kronrod_15(&integrand, wavelength_start_m, wavelength_end_m)
}

/// 2. Calculates the spectral radiance of a black body according to Planck's law.
/// 
/// # Arguments
/// * `wavelength_m` - Wavelength in meters ($\lambda$)
/// * `temperature_k` - Absolute temperature in Kelvin ($T$)
/// 
/// # Returns
/// * Spectral radiance in $\text{W} \cdot \text{m}^{-2} \cdot \text{sr}^{-1} \cdot \text{m}^{-1}$
pub fn planck_spectral_radiance(wavelength_m: f64, temperature_k: f64) -> f64 {
    // 01. Define physical constants (CODATA recommended values)
    const H: f64 = 6.626_070_15E-34; // Planck constant (J s)
    const C: f64 = 2.997_924_58E8;  // Speed of light in vacuum (m/s)
    const K_B: f64 = 1.380_649E-23;  // Boltzmann constant (J/K)

    // 02. Compute intermediate exponents to prevent unnecessary repetition
    let exponent = (H * C) / (wavelength_m * K_B * temperature_k);
    
    // 03. Calculate numerator and denominator of Planck's law formula
    let numerator = 2.0 * H * C * C;
    let denominator = wavelength_m.powi(5) * (exponent.exp() - 1.0);

    numerator / denominator
}

/// 3. Computes the emission maximum wavelength using Wien's displacement law.
/// 
/// # Arguments
/// * `temperature_k` - Absolute temperature in Kelvin ($T$)
/// 
/// # Returns
/// * Peak wavelength in meters ($\lambda_{\max}$)
pub fn wien_emission_maximum(temperature_k: f64) -> f64 {
    // 01. Wien's displacement constant ($b \approx 2.897_771_955 \times 10^{-3} \text{ m}\cdot\text{K}$)
    const WIEN_CONSTANT: f64 = 2.897_771_955E-3;
    
    WIEN_CONSTANT / temperature_k
}

/// 4. Creates a detailed engineering report for Planck radiation calculations
/// including numerically evaluated radiance integrals via the workspace `integral` crate.
/// 
/// # Arguments
/// * `path` - Target output file path (reference to `Path`)
/// * `wavelength_range_nm` - Spectral range in nanometers (e.g., 380.0..=900.0)
/// * `temperature_k` - Absolute temperature of the black body in Kelvin (e.g., 5778.0 K)
pub fn create_planck_engineering_report<P: AsRef<Path>>(
    path: P,
    wavelength_range_nm: (f64, f64),
    temperature_k: f64,
) -> core_io::Result<()> {
    // 01. Convert nanometer range to meters for physical integration
    let lambda_start_m = wavelength_range_nm.0 * 1e-9;
    let lambda_end_m = wavelength_range_nm.1 * 1e-9;

    // 02. Compute spectral radiance integral using the integral crate (GK15)
    let (integrated_radiance, estimated_error) = planck_radiance_integral_gk15(
        lambda_start_m,
        lambda_end_m,
        temperature_k,
    );

    // 03. Compute numerical emission maximum via Wien's displacement law
    let peak_wavelength_m = wien_emission_maximum(temperature_k);
    let peak_wavelength_nm = peak_wavelength_m * 1e9;
    let peak_spectral_radiance = planck_spectral_radiance(peak_wavelength_m, temperature_k);

    // 04. Define PDF layout settings using core_io pdf_handler
    let settings = core_io::pdf_handler::PdfSettings {
        title: "Planck Radiation Engineering Report".to_string(),
        author: "SciRust Optics Module".to_string(),
        page_width_mm: 210.0,
        page_height_mm: 297.0,
    };

    // 05. Compile the engineering report as structured Markdown content including integration results
    let content_markdown = format!(
        r#"# Ingenieursreport: Plancksche Strahlungsverteilung
Datum: 2026-07-27

## 1. Spezifikation & Eingangsgrößen (Inputs)
- **Spektraler Bereich (Lambda):** {:.2} nm bis {:.2} nm
- **Temperatur des Schwarzen Körpers (T):** {:.2} K

## 2. Berechnete Ausgangsgrößen (Outputs)
- **Integrierte Spektrale Strahlungsdichte (GK15):** {:.6e} $\text{{W}} \cdot \text{{m}}^{{-2}} \cdot \text{{sr}}^{{-1}}$
- **Geschätzter Integrationsfehler:** {:.6e}
- **Emissionsmaximum ($\lambda_{{\max}}$):** {:.4} nm ({:.6e} m)
- **Maximale Spektrale Strahlungsdichte:** {:.6e} $\text{{W}} \cdot \text{{m}}^{{-2}} \cdot \text{{sr}}^{{-1}} \cdot \text{{m}}^{{-1}}$
"#,
        wavelength_range_nm.0,
        wavelength_range_nm.1,
        temperature_k,
        integrated_radiance,
        estimated_error,
        peak_wavelength_nm,
        peak_wavelength_m,
        peak_spectral_radiance
    );

    // 06. Invoke the report generation function from core_io
    core_io::pdf_handler::generate_pdf_report(path, &settings, &content_markdown)
}