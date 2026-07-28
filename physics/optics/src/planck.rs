// physics/optics/src/planck.rs

//! # Planck's Law & Radiation Module (frequency-based version)
//!
//! Computes black body radiation characteristics in terms of frequency,
//! evaluates emission maximums via Wien's displacement law for frequency,
//! and integrates spectral radiance using the workspace `integral` library.

use std::path::Path;

// =============================================================================
// 1) Physical constants (CODATA recommended values, SI)
// =============================================================================

const H: f64 = 6.626_070_15e-34;       // Planck constant [J s]
const C: f64 = 2.997_924_58e8;         // Speed of light in vacuum [m/s]
const K_B: f64 = 1.380_649e-23;        // Boltzmann constant [J/K]

// =============================================================================
// 2) Planck spectral radiance in frequency form B_nu(nu, T)
// =============================================================================

/// Calculates the spectral radiance of a black body according to Planck's law
/// in frequency form.
///
/// # Arguments
/// * `frequency_hz` - Frequency in hertz ($\nu$)
/// * `temperature_k` - Absolute temperature in Kelvin ($T$)
///
/// # Returns
/// * Spectral radiance in $\text{W} \cdot \text{m}^{-2} \cdot \text{sr}^{-1} \cdot \text{Hz}^{-1}$
pub fn planck_spectral_radiance_nu(frequency_hz: f64, temperature_k: f64) -> f64 {
    if frequency_hz < 0.0 {
        return f64::NAN;
    }
    if temperature_k <= 0.0 {
        return f64::NAN;
    }
    if frequency_hz == 0.0 {
        return 0.0;
    }

    // x = h * nu / (k_B * T)
    let x = (H * frequency_hz) / (K_B * temperature_k);

    // B_nu = (2 * h * nu^3 / c^2) / (exp(x) - 1)
    let numerator = 2.0 * H * frequency_hz.powi(3);
    let denominator = C.powi(2) * x.exp_m1();

    numerator / denominator
}

// =============================================================================
// 3) Integrated spectral radiance over frequency band [nu1, nu2]
// =============================================================================

/// Calculates the integrated spectral radiance of a black body between two
/// frequencies using the workspace Gauss-Kronrod 15-point quadrature rule.
///
/// # Arguments
/// * `frequency_start_hz` - Start frequency in hertz ($\nu_1$)
/// * `frequency_end_hz` - End frequency in hertz ($\nu_2$)
/// * `temperature_k` - Absolute temperature in Kelvin ($T$)
///
/// # Returns
/// * Tuple of `(integrated_radiance, estimated_error)`
///   in $\text{W} \cdot \text{m}^{-2} \cdot \text{sr}^{-1}$
pub fn planck_radiance_integral_gk15(
    frequency_start_hz: f64,
    frequency_end_hz: f64,
    temperature_k: f64,
) -> (f64, f64) {
    // 01. Define the closure wrapper for Planck's law function
    let integrand = |nu: f64| planck_spectral_radiance_nu(nu, temperature_k);

    // 02. Invoke the Gauss-Kronrod 15-point rule from the workspace integral crate
    integral::methods::gauss_kronrod_15(&integrand, frequency_start_hz, frequency_end_hz)
}

// =============================================================================
// 4) Wien's displacement law for frequency
// =============================================================================

/// Computes the emission maximum frequency using Wien's displacement law
/// for the frequency form of Planck's law.
///
/// The peak frequency satisfies:
///     3 * (1 - exp(-x)) = x, with x = h * nu_max / (k_B * T)
///
/// Numerically: nu_max = k_nu * T, with k_nu ≈ 5.8789232e10 Hz/K
///
/// # Arguments
/// * `temperature_k` - Absolute temperature in Kelvin ($T$)
///
/// # Returns
/// * Peak frequency in hertz ($\nu_{\max}$)
pub fn wien_emission_maximum_frequency(temperature_k: f64) -> f64 {
    if temperature_k <= 0.0 {
        return f64::NAN;
    }
    // From numerical solution of 3*(1-exp(-x)) = x
    const K_NU: f64 = 5.878_923_2e10; // Hz/K
    K_NU * temperature_k
}

/// Computes the emission maximum wavelength using Wien's displacement law
/// for the **wavelength form** of Planck's law (B_lambda).
///
/// The peak wavelength satisfies:
///     5 * (1 - exp(-x)) = x, with x = h * c / (lambda_max * k_B * T)
///
/// Numerically: lambda_max = b / T, with b ≈ 2.897_771_955e-3 m·K
///
/// # Arguments
/// * `temperature_k` - Absolute temperature in Kelvin (T)
///
/// # Returns
/// * Peak wavelength in meters (lambda_max)
pub fn wien_emission_maximum_wavelength(temperature_k: f64) -> f64 {
    if temperature_k <= 0.0 {
        return f64::NAN;
    }
    // Wien's displacement constant for wavelength form
    const B_WIEN: f64 = 2.897_771_955e-3; // m·K
    B_WIEN / temperature_k
}

/// Computes the peak wavelength corresponding to the frequency-form peak.
/// This is NOT the same as the peak of B_lambda (wavelength form).
///
/// # Arguments
/// * `temperature_k` - Absolute temperature in Kelvin ($T$)
///
/// # Returns
/// * Wavelength corresponding to nu_max in meters ($\lambda = c / \nu_{\max}$)
pub fn wavelength_at_frequency_peak(temperature_k: f64) -> f64 {
    let nu_max = wien_emission_maximum_frequency(temperature_k);
    C / nu_max
}

/// 2. Calculates the spectral radiance of a black body according to Planck's law.
/// 
/// # Arguments
/// * `wavelength_m` - Wavelength in meters ($\lambda$)
/// * `temperature_k` - Absolute temperature in Kelvin ($T$)
/// 
/// # Returns
/// * Spectral radiance in $\text{W} \cdot \text{m}^{-2} \cdot \text{sr}^{-1} \cdot \text{m}^{-1}$
pub fn planck_spectral_radiance_lambda(wavelength_m: f64, temperature_k: f64) -> f64 {
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

/// Total radiance per steradian (integral over all frequencies)
/// = (sigma / pi) * T^4
pub fn total_radiance_per_steradian(temperature_k: f64) -> f64 {
    if temperature_k <= 0.0 {
        return f64::NAN;
    }
    const SIGMA: f64 = 5.670_374_419e-8; // Stefan-Boltzmann constant [W/(m^2 K^4)]
    (SIGMA / std::f64::consts::PI) * temperature_k.powi(4)
}

/// Hemispherical exitance (Stefan-Boltzmann law)
/// = sigma * T^4
pub fn stefan_boltzmann_exitance(temperature_k: f64) -> f64 {
    if temperature_k <= 0.0 {
        return f64::NAN;
    }
    const SIGMA: f64 = 5.670_374_419e-8; // Stefan-Boltzmann constant [W/(m^2 K^4)]
    SIGMA * temperature_k.powi(4)
}

// =============================================================================
// 5) Engineering report
// =============================================================================

/// Creates a detailed engineering report for Planck radiation calculations
/// including numerically evaluated radiance integrals via the workspace `integral` crate.
///
/// This version operates on a **frequency band** and explicitly reports:
/// - B_nu, B_lambda at a reference frequency (same physical point)
/// - Jacobian ratio at that point (should be ≈ 1)
/// - Total radiance/sr, Exitance
/// - Peak nu (B_nu maximum) and Peak lambda (B_lambda maximum)
/// - Band integral
///
/// # Arguments
/// * `path` - Target output file path (reference to `Path`)
/// * `frequency_range_hz` - Spectral range in hertz (e.g., (5e13, 2e14))
/// * `temperature_k` - Absolute temperature of the black body in Kelvin
pub fn create_planck_engineering_report<P: AsRef<Path>>(
    path: P,
    frequency_range_hz: (f64, f64),
    temperature_k: f64,
) -> core_io::Result<()> {
    let nu_start_hz = frequency_range_hz.0;
    let nu_end_hz = frequency_range_hz.1;

    // 01. Band integral (GK15)
    let (band_integral, estimated_error) = planck_radiance_integral_gk15(
        nu_start_hz,
        nu_end_hz,
        temperature_k,
    );

    // 02. Choose a reference frequency for B_nu, B_lambda, Jacobian
    // Example: use the start frequency as reference (analogous to Python)
    let nu_ref = nu_start_hz;
    let lambda_ref_m = C / nu_ref;

    let b_nu_ref = planck_spectral_radiance_nu(nu_ref, temperature_k);
    let b_lambda_ref = planck_spectral_radiance_lambda(lambda_ref_m, temperature_k);

    // Jacobian ratio at reference point:
    // B_lambda = B_nu * |dnu/dlambda| = B_nu * c / lambda^2
    let jacobian_ratio = if b_nu_ref != 0.0 && lambda_ref_m != 0.0 {
        b_lambda_ref / (b_nu_ref * C / lambda_ref_m.powi(2))
    } else {
        f64::NAN
    };

    // 03. Peak frequency (B_nu maximum)
    let peak_nu = wien_emission_maximum_frequency(temperature_k);
    let b_nu_peak = planck_spectral_radiance_nu(peak_nu, temperature_k);

    // 04. Peak wavelength (B_lambda maximum, separate from peak_nu)
    let peak_lambda_m = wien_emission_maximum_wavelength(temperature_k);
    let peak_lambda_nm = peak_lambda_m * 1e9;
    let b_lambda_peak = planck_spectral_radiance_lambda(peak_lambda_m, temperature_k);

    // 05. Total radiance per steradian and exitance (Stefan-Boltzmann)
    let total_radiance_per_sr = total_radiance_per_steradian(temperature_k);
    let exitance = stefan_boltzmann_exitance(temperature_k);

    // 06. PDF settings
    let settings = core_io::pdf_handler::PdfSettings {
        title: "Planck Radiation Engineering Report (frequency basis)".to_string(),
        author: "SciRust Optics Module".to_string(),
        page_width_mm: 210.0,
        page_height_mm: 297.0,
    };

    // 07. Markdown report
    let content_markdown = format!(
        r#"# Ingenieursreport: Plancksche Strahlungsverteilung (Frequenzbasis)
Datum: 2026-07-28


## 1. Spezifikation & Eingangsgrößen (Inputs)
- **Spektraler Bereich (Frequenz):** {:.6e} Hz bis {:.6e} Hz
- **Temperatur des Schwarzen Körpers (T):** {:.2} K
- **Referenzfrequenz (nu_ref):** {:.6e} Hz
- **Referenzwellenlänge (lambda_ref):** {:.6e} m ({:.4} nm)


## 2. Berechnete Ausgangsgrößen (Outputs)
- **B_nu (Referenz):** {:.6e} W/(m^2 sr Hz)
- **B_lambda (Referenz):** {:.6e} W/(m^2 sr m)
- **Jacobian ratio (Referenz):** {:.6e}
- **B_nu (Peak, nu_max):** {:.6e} W/(m^2 sr Hz)
- **B_lambda (Peak, lambda_max):** {:.6e} W/(m^2 sr m)
- **Total radiance/sr:** {:.6e} W/(m^2 sr)
- **Exitance:** {:.6e} W/m^2
- **Peak nu:** {:.6e} Hz
- **Peak lambda:** {:.6e} m ({:.4} nm)
- **Band integral:** {:.6e} W/(m^2 sr)
- **Geschätzter Integrationsfehler:** {:.6e}
"#,
        nu_start_hz,
        nu_end_hz,
        temperature_k,
        nu_ref,
        lambda_ref_m,
        lambda_ref_m * 1e9,
        b_nu_ref,
        b_lambda_ref,
        jacobian_ratio,
        b_nu_peak,
        b_lambda_peak,
        total_radiance_per_sr,
        exitance,
        peak_nu,
        peak_lambda_m,
        peak_lambda_nm,
        band_integral,
        estimated_error,
    );

    // 08. Generate PDF
    core_io::pdf_handler::generate_pdf_report(path, &settings, &content_markdown)
}

/*
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
    let integrand = |lam: f64| planck_spectral_radiance_lambda(lam, temperature_k);

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
pub fn planck_spectral_radiance_lambda(wavelength_m: f64, temperature_k: f64) -> f64 {
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
    let peak_spectral_radiance = planck_spectral_radiance_lambda(peak_wavelength_m, temperature_k);

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
*/