//! # Mathematical Constants Module
//! 
//! Central repository for fundamental mathematical constants used across the SciRust workspace.

/// Zentraler Container für mathematische Konstanten mit Scoping-Erzwingung.
pub struct MathConstants {
    // Verhindert das Erstellen der Struktur außerhalb dieser Datei vollständig
    _private: (), 
}

impl MathConstants {
    // =========================================================================
    // Machine Constants 
    // =========================================================================

    pub const MACHINE_EPS_F64: f64 = f64::EPSILON;
    pub const MACHINE_EPS_F32: f32 = f32::EPSILON;
    pub const MAX_U128: u128 = u128::MAX;
    pub const MAX_I128: i128 = i128::MAX;
    pub const MIN_I128: i128 = i128::MIN;
    pub const MAX_U64: u64 = u64::MAX;
    pub const MAX_I64: i64 = i64::MAX;
    pub const MIN_I64: i64 = i64::MIN;
    pub const MAX_U32: u32 = u32::MAX; // Korrigiert
    pub const MAX_I32: i32 = i32::MAX; // Korrigiert
    pub const MIN_I32: i32 = i32::MIN; // Korrigiert

    // =========================================================================
    // Minimal Tolerance
    // =========================================================================
    pub const MIN_ABSTOL_F64: f64 = Self::MACHINE_EPS_F64 * 0.001;
    pub const MIN_RELTOL_F64: f64 = Self::MACHINE_EPS_F64 * 0.001;

    // =========================================================================
    // Fundamental Constants
    // =========================================================================

    /// Archimedes' constant (Pi, $\pi$).
    pub const PI: f64 = std::f64::consts::PI;

    /// Euler's number ($e$, base of natural logarithms).
    pub const E: f64 = std::f64::consts::E;

    /// Square root of 2 ($\sqrt{2}$).
    pub const SQRT_2: f64 = std::f64::consts::SQRT_2;

    /// Square root of 3 ($\sqrt{3}$).
    pub const SQRT_3: f64 = 1.732_050_807_568_877_293_527_446_341_505_9;

    /// Inverse square root of 2 ($1 / \sqrt{2}$).
    pub const FRAC_1_SQRT_2: f64 = std::f64::consts::FRAC_1_SQRT_2;

    // =========================================================================
    // Geometric & Algebraic Constants
    // =========================================================================

    /// Golden ratio ($\phi = \frac{1 + \sqrt{5}}{2}$).
    pub const GOLDEN_RATIO: f64 = 1.618_033_988_749_895;

    /// Euler-Mascheroni constant ($\gamma$).
    pub const EULER_MASCHERONI: f64 = 0.577_215_664_901_532_9;

    /// Catalan's constant ($G$).
    pub const CATALAN: f64 = 0.915_965_594_177_219_0;

    /// Apéry's constant ($\zeta(3)$).
    pub const APERY: f64 = 1.202_056_903_159_594_2;

    // =========================================================================
    // Trigonometric Fractions
    // =========================================================================

    /// Pi divided by 2 ($\frac{\pi}{2}$).
    pub const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;

    /// Pi divided by 3 ($\frac{\pi}{3}$).
    pub const FRAC_PI_3: f64 = std::f64::consts::FRAC_PI_3;

    /// Pi divided by 4 ($\frac{\pi}{4}$).
    pub const FRAC_PI_4: f64 = std::f64::consts::FRAC_PI_4;

    /// Two times Pi ($2\pi$).
    pub const TWO_PI: f64 = std::f64::consts::TAU;

    // =========================================================================
    // Conversion Factors
    // =========================================================================

    /// Factor to convert degrees to radians ($\frac{\pi}{180}$).
    pub const DEG_TO_RAD: f64 = 0.017_453_292_519_943_295;

    /// Factor to convert radians to degrees ($\frac{180}{\pi}$).
    pub const RAD_TO_DEG: f64 = 57.295_779_513_082_32;
}
