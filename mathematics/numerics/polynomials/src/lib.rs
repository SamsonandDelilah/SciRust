//! Reine Polynom-Engine für SciRust (SSOT).
//! Kapselt Koeffizienten und stellt Methoden via methods.rs bereit.

pub mod methods;

use num_traits::{Float, FromPrimitive};

/// Verfügbare Algorithmen zur Polynomauswertung
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EvaluationStrategy {
    #[default]
    Horner,
    //Estrin,
    Chebychev,
}

/// Ein Polynom in Koeffizientendarstellung: p(x) = c[0] + c[1]*x + c[2]*x^2 + ... + c[n]*x^n
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T> {
    pub coefficients: Vec<T>, // c[0] ist der konstante Term
}

impl<T> Polynomial<T>
where
    T: Float + FromPrimitive,
{
    /// Erzeugt ein neues Polynom aus einer Liste von Koeffizienten
    pub fn new(coefficients: Vec<T>) -> Self {
        Self { coefficients }
    }

    /// Wertet das Polynom an der Stelle x aus (Standard: Horner, nutzt den SIMD-Dispatcher)
pub fn evaluate(&self, x: T) -> T 
    where
        T: methods::SimdEvaluator,
    {
        T::evaluate_accelerated(&self.coefficients, x)
    }

    /// Wertet das Polynom mit einer wählbaren Auswertungs-Strategie aus
pub fn evaluate_with_strategy(&self, x: T, strategy: EvaluationStrategy) -> T 
    where
        T: methods::SimdEvaluator, // <--- Das fehlte hier
    {
        match strategy {
            // Richtig: Aufruf über T statt über das methods-Modul!
            EvaluationStrategy::Horner => T::evaluate_accelerated(&self.coefficients, x),
            // Falls Estrin noch separat läuft oder auch ins Trait soll:
            //EvaluationStrategy::Estrin => methods::evaluate_estrin(&self.coefficients, x),
            // Chebychev with Clenshaw polynom:
            EvaluationStrategy::Chebychev => T::evaluate_chebyshev(&self.coefficients, x),
        }
    }

    /// Berechnet Funktionswert und erste Ableitung gleichzeitig
    pub fn evaluate_with_derivative(&self, x: T) -> (T, T) {
        methods::evaluate_with_derivative(&self.coefficients, x)
    }

    /// Gibt die formale Ableitung dieses Polynoms als neues Polynom zurück
    pub fn derivative(&self) -> Self {
        Self {
            coefficients: methods::differentiate_coefficients(&self.coefficients),
        }
    }
}


//
//  --- unit tests ---
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_struct_api() {
        // p(x) = 1 + 2x + 3x^2
        let poly = Polynomial::new(vec![1.0, 2.0, 3.0]);

        // Standard-Auswertung (Horner)
        assert_eq!(poly.evaluate(2.0), 17.0);

        // Auswertung mit expliziter Strategie
        assert_eq!(
            poly.evaluate_with_strategy(2.0, EvaluationStrategy::Horner),
            17.0
        );
        //assert_eq!(
        //    poly.evaluate_with_strategy(2.0, EvaluationStrategy::Estrin),
        //    17.0
        //);
    }

    #[test]
    fn test_polynomial_derivative_struct() {
        // p(x) = 4 + 0x + 3x^2  =>  p'(x) = 0 + 6x
        let poly = Polynomial::new(vec![4.0, 0.0, 3.0]);
        let deriv = poly.derivative();

        assert_eq!(deriv.coefficients, vec![0.0, 6.0]);
        assert_eq!(deriv.evaluate(3.0), 18.0);
    }

    #[test]
    fn test_simultaneous_evaluation_struct() {
        // p(x) = 1 + 2x + 3x^2
        let poly = Polynomial::new(vec![1.0, 2.0, 3.0]);
        let (val, deriv) = poly.evaluate_with_derivative(2.0);

        assert_eq!(val, 17.0);
        assert_eq!(deriv, 14.0); // p'(x) = 2 + 6x => p'(2) = 14
    }

// Eine kleine Hilfsfunktion zum Vergleichen von Gleitkommazahlen mit Toleranz
    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn test_chebyshev_evaluation() {
        // Beispiel: T_0(x) = 1, T_1(x) = x, T_2(x) = 2x^2 - 1
        // Koeffizienten: c = [1.0, 0.0, 1.0] bedeutet: 1*T_0 + 0*T_1 + 1*T_2
        // Bei x = 0.5: T_0(0.5) = 1.0, T_2(0.5) = 2*(0.5)^2 - 1 = 2*0.25 - 1 = -0.5
        // Erwartetes Ergebnis: 1.0 + (-0.5) = 0.5
        let poly = Polynomial::new(vec![1.0, 0.0, 1.0]);
        let result = poly.evaluate_with_strategy(0.5, EvaluationStrategy::Chebychev);
        
        assert!(approx_eq(result, 0.5, 1e-10), "Erwartet 0.5, erhalten {}", result);
    }

    #[test]
    fn test_strategy_dispatch() {
        let poly = Polynomial::new(vec![1.0, 2.0, 3.0]);
        
        // Prüfen, ob die Strategien ohne Panics durchlaufen
        let res_horner = poly.evaluate_with_strategy(0.5, EvaluationStrategy::Horner);
        let res_cheby = poly.evaluate_with_strategy(0.5, EvaluationStrategy::Chebychev);
        
        // Horner rechnet Potenzreihe, Chebychev Chebyshev-Reihe – die Ergebnisse müssen sich unterscheiden
        assert_ne!(res_horner, res_cheby);
    }

}