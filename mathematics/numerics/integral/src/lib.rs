pub mod methods;

use methods::{
    adaptive_quadrature, ee_integral, symbolic_integral, 
    trapezoidal_integral, simpson_integral, romberg_integral,
    discrete_vector_integral
};
use num_traits::{Float, FromPrimitive};

/// Das Kontrollzentrum für die gezielte Verfahrenswahl in SciRust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SolverStrategy {
    #[default]
    Auto,
    GaussKronrod15,
    DoubleExponential,
    SymbolicCAS,
    Trapezoidal,       // Schnelle Trapez-Abschätzung (fest: 100 Schritte)
    Simpson,           // Schnelle Simpson-Abschätzung (fest: 100 Schritte)
    Romberg,           // Romberg-Verfahren
    
    #[cfg(feature = "use_quadrature")]
    ExternalQuadrature, 
}

pub trait Integrable<T> {
    fn eval(&self, x: T) -> T;
}

impl<T, F: Fn(T) -> T> Integrable<T> for F {
    #[inline]
    fn eval(&self, x: T) -> T { self(x) }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntegrationResult<T> {
    Ok { value: T, method: &'static str },
    Err { reason: &'static str },
}

/// Die universelle Plattform-Schnittstelle von SciRust (Single-Threaded optimiert).
pub fn integrate_with_strategy<T, I>(
    integrand: I, 
    expression: &str, 
    start: T, 
    end: T, 
    strategy: SolverStrategy
) -> IntegrationResult<T>
where
    T: Float + FromPrimitive + Send + Sync,
    I: Integrable<T> + Sync,
{
    let tol = T::from_f64(1e-8).unwrap();

    if start > end { return IntegrationResult::Err { reason: "Ungültiges Intervall: Startwert > Endwert." }; }
    if start == end { return IntegrationResult::Ok { value: T::zero(), method: "Analytischer Intervall-Null-Check" }; }

    match strategy {
        SolverStrategy::Auto => {
            let safe_start = if start.is_zero() { T::from_f64(1e-12).unwrap() } else { start };
            
            // 1. VERSUCH: Schnelles und robustes ee_integral für Singularitäten
            let res_ee = ee_integral(&|x| integrand.eval(x), safe_start, end, tol);
            if res_ee.is_finite() && !res_ee.is_nan() {
                return IntegrationResult::Ok { value: res_ee, method: "SciRust ee_integral (Automatische Auswahl)" };
            }

            // 2. VERSUCH (Fallback): Adaptives Gauß-Kronrod
            let res_gk = adaptive_quadrature(&|x| integrand.eval(x), safe_start, end, tol, 20);
            if res_gk.is_finite() && !res_gk.is_nan() {
                return IntegrationResult::Ok { value: res_gk, method: "SciRust Adaptive Gauß-Kronrod (Fallback)" };
            }

            IntegrationResult::Err { reason: "Numerische Instabilität: Kein integriertes Verfahren konvergiert." }
        },
        SolverStrategy::GaussKronrod15 => {
            let res = adaptive_quadrature(&|x| integrand.eval(x), start, end, tol, 20);
            IntegrationResult::Ok { value: res, method: "Native Gauß-Kronrod 15" }
        },
        SolverStrategy::DoubleExponential => {
            let res = ee_integral(&|x| integrand.eval(x), start, end, tol);
            IntegrationResult::Ok { value: res, method: "Native ee_integral" }
        },
        SolverStrategy::SymbolicCAS => {
            match symbolic_integral(expression, start, end) {
                Ok(res) => IntegrationResult::Ok { value: res, method: "SciRust Analytical CAS Engine" },
                Err(err) => IntegrationResult::Err { reason: err },
            }
        },
                SolverStrategy::Trapezoidal => {
            let res = trapezoidal_integral(&|x| integrand.eval(x), start, end, 100);
            IntegrationResult::Ok { value: res, method: "SciRust Composite Trapezoidal (100 Steps)" }
        },
        SolverStrategy::Simpson => {
            let res = simpson_integral(&|x| integrand.eval(x), start, end, 100);
            IntegrationResult::Ok { value: res, method: "SciRust Composite Simpson (100 Steps)" }
        },
        SolverStrategy::Romberg => {
            let res = romberg_integral(&|x| integrand.eval(x), start, end, tol);
            IntegrationResult::Ok { value: res, method: "SciRust Romberg Integration" }
        },

        #[cfg(feature = "use_quadrature")]
        SolverStrategy::ExternalQuadrature => {
            let res = quadrature::integrate(|x| integrand.eval(T::from_f64(x).unwrap()).to_f64().unwrap(), start.to_f64().unwrap(), end.to_f64().unwrap(), tol.to_f64().unwrap());
            IntegrationResult::Ok { value: T::from_f64(res.integral).unwrap(), method: "Externes Crate: quadrature" }
        }
       
    }
}

pub fn integrate<T, I>(integrand: I, start: T, end: T) -> IntegrationResult<T>
where
    T: Float + FromPrimitive + Send + Sync,
    I: Integrable<T> + Sync,
{
    integrate_with_strategy(integrand, "", start, end, SolverStrategy::Auto)
}

/// Universelle Plattform-Schnittstelle für diskrete Messdaten-Vektoren.
///
/// Wertet reale Messreihen (z.B. Cp-Werte über Temperaturpunkte) autonom aus.
pub fn integrate_vectors<T>(x: &[T], y: &[T]) -> IntegrationResult<T>
where
    T: Float + FromPrimitive + Send + Sync,
{
    match discrete_vector_integral(x, y) {
        Ok(val) => IntegrationResult::Ok { value: val, method: "SciRust Discrete Vector Engine" },
        Err(err) => IntegrationResult::Err { reason: err },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_generics_integration() {
        let integrand = |x: f64| (-x / 5.0).exp() * x.powf(-1.0 / 3.0);
        match integrate(integrand, 0.0, 10.0) {
            IntegrationResult::Ok { value, .. } => assert!((value - 3.6798142583691758).abs() <= 1e-4),
            IntegrationResult::Err { reason } => panic!("Test fehlgeschlagen: {}", reason),
        }
    }
}