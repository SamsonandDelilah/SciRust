//! Native und generische Implementierung von Zustandsgleichungen (Equations of State) für SciRust.
//! Alle Eingaben und Ausgaben erfolgen strikt in SI-Einheiten (Pa, K, m³/mol).

use num_traits::{Float, FromPrimitive};

/// Konstante für die universelle Gaskonstante R in J/(mol·K)
pub const R_IDEAL: f64 = 8.314462618;

/// Das fundamentale Trait für alle Gasmodelle in SciRust.
pub trait GasModel<T: Float + FromPrimitive> {
    /// Berechnet den Druck p in Pascal (Pa) für eine gegebene Temperatur T (K) und molares Volumen v (m³/mol).
    fn calculate_pressure(&self, temperature: T, molar_volume: T) -> T;
}

// -------------------------------------------------------------------------
// 1) IDEALES GAS
// -------------------------------------------------------------------------
pub struct IdealGas<T> {
    pub r: T,
}

impl<T: Float + FromPrimitive> Default for IdealGas<T> {
    fn default() -> Self {
        Self { r: T::from_f64(R_IDEAL).unwrap() }
    }
}

impl<T: Float + FromPrimitive> GasModel<T> for IdealGas<T> {
    #[inline]
    fn calculate_pressure(&self, temperature: T, molar_volume: T) -> T {
        (self.r * temperature) / molar_volume
    }
}

// -------------------------------------------------------------------------
// 2) VAN-DER-WAALS (VDW)
// -------------------------------------------------------------------------
pub struct VanDerWaals<T> {
    pub a: T, // Kohäsionsdruck-Parameter (Pa·m⁶/mol²)
    pub b: T, // Kovolumen-Parameter (m³/mol)
    pub r: T,
}

impl<T: Float + FromPrimitive> VanDerWaals<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { a, b, r: T::from_f64(R_IDEAL).unwrap() }
    }
}

impl<T: Float + FromPrimitive> GasModel<T> for VanDerWaals<T> {
    #[inline]
    fn calculate_pressure(&self, temperature: T, molar_volume: T) -> T {
        (self.r * temperature) / (molar_volume - self.b) - self.a / (molar_volume * molar_volume)
    }
}

// -------------------------------------------------------------------------
// 3) REDLICH-KWONG (RK)
// -------------------------------------------------------------------------
pub struct RedlichKwong<T> {
    pub a: T,
    pub b: T,
    pub r: T,
}

impl<T: Float + FromPrimitive> RedlichKwong<T> {
    pub fn new(tc: T, pc: T) -> Self {
        let r = T::from_f64(R_IDEAL).unwrap();
        let a = T::from_f64(0.42748).unwrap() * (r * r * tc.powf(T::from_f64(2.5).unwrap())) / pc;
        let b = T::from_f64(0.08664).unwrap() * (r * tc) / pc;
        Self { a, b, r }
    }
}

impl<T: Float + FromPrimitive> GasModel<T> for RedlichKwong<T> {
    #[inline]
    fn calculate_pressure(&self, temperature: T, molar_volume: T) -> T {
        let r = self.r;
        let denominator = molar_volume * (molar_volume + self.b) * temperature.sqrt();
        (r * temperature) / (molar_volume - self.b) - self.a / denominator
    }
}

// -------------------------------------------------------------------------
// 4) SOAVE-REDLICH-KWONG (SRK)
// -------------------------------------------------------------------------
pub struct SoaveRedlichKwong<T> {
    pub tc: T,
    pub omega: T,
    pub a: T,
    pub b: T,
    pub r: T,
}

impl<T: Float + FromPrimitive> SoaveRedlichKwong<T> {
    pub fn new(tc: T, pc: T, omega: T) -> Self {
        let r = T::from_f64(R_IDEAL).unwrap();
        let a = T::from_f64(0.42747).unwrap() * (r * r * tc * tc) / pc;
        let b = T::from_f64(0.08664).unwrap() * (r * tc) / pc;
        Self { tc, omega, a, b, r }
    }

    fn alpha(&self, temperature: T) -> T {
        let tr = temperature / self.tc;
        let m = T::from_f64(0.48).unwrap() 
            + T::from_f64(1.574).unwrap() * self.omega 
            - T::from_f64(0.176).unwrap() * self.omega * self.omega;
        (T::one() + m * (T::one() - tr.sqrt())).powi(2)
    }
}

impl<T: Float + FromPrimitive> GasModel<T> for SoaveRedlichKwong<T> {
    #[inline]
    fn calculate_pressure(&self, temperature: T, molar_volume: T) -> T {
        let a_t = self.a * self.alpha(temperature);
        (self.r * temperature) / (molar_volume - self.b) - a_t / (molar_volume * (molar_volume + self.b))
    }
}

// -------------------------------------------------------------------------
// 5) PENG-ROBINSON (PR)
// -------------------------------------------------------------------------
pub struct PengRobinson<T> {
    pub tc: T,
    pub omega: T,
    pub a: T,
    pub b: T,
    pub r: T,
}

impl<T: Float + FromPrimitive> PengRobinson<T> {
    pub fn new(tc: T, pc: T, omega: T) -> Self {
        let r = T::from_f64(R_IDEAL).unwrap();
        let a = T::from_f64(0.45724).unwrap() * (r * r * tc * tc) / pc;
        let b = T::from_f64(0.07780).unwrap() * (r * tc) / pc;
        Self { tc, omega, a, b, r }
    }

    fn alpha(&self, temperature: T) -> T {
        let tr = temperature / self.tc;
        let m = T::from_f64(0.37464).unwrap() 
            + T::from_f64(1.54226).unwrap() * self.omega 
            - T::from_f64(0.26992).unwrap() * self.omega * self.omega;
        (T::one() + m * (T::one() - tr.sqrt())).powi(2)
    }
}

impl<T: Float + FromPrimitive> GasModel<T> for PengRobinson<T> {
    #[inline]
    fn calculate_pressure(&self, temperature: T, molar_volume: T) -> T {
        let a_t = self.a * self.alpha(temperature);
        let b = self.b;
        let denominator = molar_volume * (molar_volume + b) + b * (molar_volume - b);
        (self.r * temperature) / (molar_volume - b) - a_t / denominator
    }
}

// -------------------------------------------------------------------------
// 6) VIRIALGLEICHUNG
// -------------------------------------------------------------------------
pub struct VirialEquation<T> {
    pub b_coeff: T, // 2. Virialkoeffizient (m³/mol)
    pub c_coeff: T, // 3. Virialkoeffizient (m⁶/mol²)
    pub r: T,
}

impl<T: Float + FromPrimitive> VirialEquation<T> {
    pub fn new(b_coeff: T, c_coeff: T) -> Self {
        Self { b_coeff, c_coeff, r: T::from_f64(R_IDEAL).unwrap() }
    }
}

impl<T: Float + FromPrimitive> GasModel<T> for VirialEquation<T> {
    #[inline]
    fn calculate_pressure(&self, temperature: T, molar_volume: T) -> T {
        let term = T::one() + self.b_coeff / molar_volume + self.c_coeff / (molar_volume * molar_volume);
        (self.r * temperature) / molar_volume * term
    }
}

// -------------------------------------------------------------------------
// 7) ALLGEMEINE KUBISCHE ZUSTANDSGLEICHUNG (C3)
// -------------------------------------------------------------------------
pub struct CubicEOS<T> {
    pub a: T,
    pub b: T,
    pub u: T,
    pub w: T,
    pub r: T,
}

impl<T: Float + FromPrimitive> CubicEOS<T> {
    pub fn new(a: T, b: T, u: T, w: T) -> Self {
        Self { a, b, u, w, r: T::from_f64(R_IDEAL).unwrap() }
    }
}

impl<T: Float + FromPrimitive> GasModel<T> for CubicEOS<T> {
    #[inline]
    fn calculate_pressure(&self, temperature: T, molar_volume: T) -> T {
        let b = self.b;
        let denominator = molar_volume * molar_volume + self.u * b * molar_volume + self.w * b * b;
        (self.r * temperature) / (molar_volume - b) - self.a / denominator
    }
}



    