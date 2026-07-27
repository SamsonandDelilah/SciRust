// mathematics/math_constants.rs

/// 1. Pros and Cons analysis of centralizing mathematical constants in a dedicated module.
/// 
/// ## Pro
/// * **Single Source of Truth:** Central definition prevents magic numbers and duplication across crates (e.g., geometry, integration, symbolic).
/// * **Maintainability & Consistency:** Changing precision or switching to custom numeric types (like Bigfloat / `pln4`) only requires updates in one file.
/// * **Readability:** Clear, domain-specific names (`MathConstants::PI`) improve code clarity over scattered `std::f64::consts::PI`.
/// * **Extensibility:** The `#[non_exhaustive]` attribute allows adding custom constants later without breaking downstream code.
/// 
/// ## Con
/// * **Minor Indirection:** Requires importing or referencing the constants module, adding a slight verbosity compared to standard library constants.
/// * **Namespace Pollution Risk:** If imported globally via `use MathConstants::*;`, it can occasionally cause naming collisions with local variables.
/// ## Architectural Alignment
/// * **Enforcing Explicit Scope:** Rejecting glob imports (`use MathConstants::*;`) and requiring explicit qualification (`MathConstants::PI`) prevents namespace pollution.
/// * **Code Clarity:** Explicit prefixes make the origin of numerical constants unambiguous, aligning with strict software engineering practices in high-precision scientific computing libraries like SciRust.
/// 
//! # Mathematical Constants Module
//! 
//! Central repository for fundamental mathematical constants used across the SciRust workspace.

/// 1. Central mathematical constants container with explicit scoping enforcement.
#[non_exhaustive]
pub struct MathConstants;

impl MathConstants {
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
    pub const SQRT_3: f64 = std::f64::consts::SQRT_3;

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
    pub const TAU: f64 = std::f64::consts::TAU;

    // =========================================================================
    // Conversion Factors
    // =========================================================================

    /// Factor to convert degrees to radians ($\frac{\pi}{180}$).
    pub const DEG_TO_RAD: f64 = 0.017_453_292_519_943_295;

    /// Factor to convert radians to degrees ($\frac{180}{\pi}$).
    pub const RAD_TO_DEG: f64 = 57.295_779_513_082_32;
}