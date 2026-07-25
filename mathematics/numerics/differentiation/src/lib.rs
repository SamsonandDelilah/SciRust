pub mod methods;

use methods::{
    forward_difference, backward_difference, central_difference,
    high_order_central_5, adaptive_differentiation
};
use num_traits::{Float, FromPrimitive};

/// Strategie-Auswahl für numerische Ableitungen in SciRust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DifferentiationStrategy {
    #[default]
    Auto,
    Forward,
    Backward,
    Central,          // O(h^2)
    HighOrder5,       // O(h^4) - SciPy/Boost Standard
    Adaptive,         // Richardson-extrapoliert mit Fehlerabschätzung
    ComplexStep,      // Höchste Präzision via komplexer Schrittweite
}

pub trait Differentiable<T> {
    fn eval(&self, x: T) -> T;
}

impl<T, F: Fn(T) -> T> Differentiable<T> for F {
    #[inline]
    fn eval(&self, x: T) -> T { self(x) }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DifferentiationResult<T> {
    Ok { value: T, error_estimate: T, method: &'static str },
    Err { reason: &'static str },
}

/// Die universelle Plattform-Schnittstelle für Ableitungen in SciRust.
pub fn differentiate_with_strategy<T, D>(
    function: D,
    x: T,
    strategy: DifferentiationStrategy,
    step_size: Option<T>,
) -> DifferentiationResult<T>
where
    T: Float + FromPrimitive + Send + Sync,
    D: Differentiable<T> + Sync,
{
    // Automatische Bestimmung einer robusten Schrittweite h, falls nicht übergeben
    // Faustregel für Maschinenpräzision: h = eps^(1/3) für Central, eps^(1/4) für HighOrder
    let h = match step_size {
        Some(val) => val,
        None => {
            let eps = T::epsilon();
            eps.powf(T::from_f64(1.0 / 3.0).unwrap())
        }
    };

    match strategy {
        DifferentiationStrategy::Auto | DifferentiationStrategy::Central => {
            let val = central_difference(&|val| function.eval(val), x, h);
            DifferentiationResult::Ok {
                value: val,
                error_estimate: h * h, // O(h^2) Abschätzung
                method: "Central Difference O(h^2)",
            }
        }
        DifferentiationStrategy::Forward => {
            let val = forward_difference(&|val| function.eval(val), x, h);
            DifferentiationResult::Ok {
                value: val,
                error_estimate: h,
                method: "Forward Difference O(h)",
            }
        }
        DifferentiationStrategy::Backward => {
            let val = backward_difference(&|val| function.eval(val), x, h);
            DifferentiationResult::Ok {
                value: val,
                error_estimate: h,
                method: "Backward Difference O(h)",
            }
        }
        DifferentiationStrategy::HighOrder5 => {
            let val = high_order_central_5(&|val| function.eval(val), x, h);
            DifferentiationResult::Ok {
                value: val,
                error_estimate: h.powi(4),
                method: "High-Order 5-Point Central O(h^4)",
            }
        }
        DifferentiationStrategy::Adaptive => {
            let (val, err) = adaptive_differentiation(&|val| function.eval(val), x);
            DifferentiationResult::Ok {
                value: val,
                error_estimate: err,
                method: "Adaptive Richardson Extrapolation",
            }
        }
        DifferentiationStrategy::ComplexStep => {
            // Hinweis: Die Funktion muss hier für Complex<T> ausgelegt sein 
            // oder über eine Brücke aufgerufen werden.
            // (Für rein reelle Closures f: Fn(T) -> T lässt sich das auch via analytischer Fortsetzung nutzen, 
            // wenn die Funktion elementar ist).
            panic!("ComplexStep erfordert eine Funktion vom Typ Fn(Complex<T>) -> Complex<T>");
        }
    }    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_differentiation_methods() {
        // Beispiel: f(x) = x^2, Ableitung bei x = 3.0 sollte 6.0 sein
        let f = |x: f64| x * x;
        let res = differentiate_with_strategy(
            f,
            3.0,
            DifferentiationStrategy::Central,
            None,
        );

        match res {
            DifferentiationResult::Ok { value, .. } => {
                assert!((value - 6.0).abs() < 1e-5);
            }
            _ => panic!("Test fehlgeschlagen"),
        }
    }
}