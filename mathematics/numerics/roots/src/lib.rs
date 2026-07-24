pub mod methods;

use methods::{brent_root, newton_raphson, symbolic_root, toms748_root};
use num_traits::{Float, FromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SolverStrategy {
    #[default]
    Auto,              // Die smarte SciRust-Zweistufen-Rakete (Standard)
    Brent,             // Rein das optimierte Brent-Verfahren
    NewtonRaphson,     // Rein das schnelle Newton-Raphson-Verfahren
    Toms748,           
    Symbolic,          
}

pub trait RootFunction<T> {
    fn eval(&self, x: T) -> T;
}

impl<T, F: Fn(T) -> T> RootFunction<T> for F {
    #[inline]
    fn eval(&self, x: T) -> T { self(x) }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RootsResult<T> {
    Ok { value: T, method: &'static str },
    Err { reason: &'static str },
}

/// Die universelle Plattform-Schnittstelle von SciRust (Zweistufen-Rakete)
pub fn find_root_with_strategy<T, F, DF>(
    integrand: F,
    f_prime: Option<DF>,
    expression: &str,
    start: T,
    end: T,
    strategy: SolverStrategy,
) -> RootsResult<T>
where
    T: Float + FromPrimitive,
    F: RootFunction<T>,
    DF: RootFunction<T>,
{
    // Stufe 1 Toleranz (Grobeingrenzung) und Stufe 2 Toleranz (Finale Präzision)
    let tol_coarse = T::from_f64(1e-4).unwrap();
    let tol_fine = T::from_f64(1e-12).unwrap();
    let max_iter = 100;

    match strategy {
        SolverStrategy::Auto => {
            let fa = integrand.eval(start);
            let fb = integrand.eval(end);

            if fa * fb <= T::zero() {
                // 🚀 RAKETEN-STUFE 1: Unzerstörbare Grobeingrenzung via Brent bis 1e-4
                let coarse_root = match brent_root(&|x| integrand.eval(x), start, end, tol_coarse, max_iter) {
                    Ok(val) => val,
                    Err(err) => return RootsResult::Err { reason: err },
                };

                // 🚀 RAKETEN-STUFE 2: Hochpräzisions-Finish bis 1e-12
                if let Some(ref df) = f_prime {
                    // Fall A: Analytische Ableitung ist da -> Echtes Newton-Raphson zünden
                    match newton_raphson(&|x| integrand.eval(x), &|x| df.eval(x), coarse_root, tol_fine, 20) {
                        Ok(val) => RootsResult::Ok { value: val, method: "SciRust Zweistufen-Rakete (Brent + Analytisches NR)" },
                        Err(_) => RootsResult::Ok { value: coarse_root, method: "SciRust Rakete (NR-Finish fehlgeschlagen, nutze Brent-Grobwert)" },
                    }
                } else {
                    // Fall C: Keine Ableitung da -> Numerischer Joker: Sekanten-Verfahren (Quasi-Newton)
                    // Wir schätzen die Steigung lokal über einen winzigen Schritt ab (h = 1e-5)
                    let h = T::from_f64(1e-5).unwrap();
                    let num_derivative = |x: T| {
                        let fx_plus = integrand.eval(x + h);
                        let fx_minus = integrand.eval(x - h);
                        (fx_plus - fx_minus) / (T::from_f64(2.0).unwrap() * h)
                    };

                    match newton_raphson(&|x| integrand.eval(x), &num_derivative, coarse_root, tol_fine, 20) {
                        Ok(val) => RootsResult::Ok { value: val, method: "SciRust Zweistufen-Rakete (Brent + Numerisches Quasi-Newton)" },
                        Err(_) => {
                            // Letzter Fallback: Wenn das Finish fehlschlägt, jagen wir Brent komplett bis 1e-12 durch
                            match brent_root(&|x| integrand.eval(x), start, end, tol_fine, max_iter) {
                                Ok(val) => RootsResult::Ok { value: val, method: "SciRust Rakete (Fallback: Rein numerisches Brent-Deep-Finish)" },
                                Err(err) => RootsResult::Err { reason: err },
                            }
                        }
                    }
                }
            } else {
                RootsResult::Err { reason: "SciRust Automatik abgebrochen: Kein Vorzeichenwechsel im Intervall." }
            }
        },
        SolverStrategy::Brent => {
            match brent_root(&|x| integrand.eval(x), start, end, tol_fine, max_iter) {
                Ok(val) => RootsResult::Ok { value: val, method: "Native Brent-Engine" },
                Err(err) => RootsResult::Err { reason: err },
            }
        },
        SolverStrategy::NewtonRaphson => {
            if let Some(df) = f_prime {
                let zero_five = T::from_f64(0.5).unwrap();
                let guess = start + zero_five * (end - start);
                match newton_raphson(&|x| integrand.eval(x), &|x| df.eval(x), guess, tol_fine, max_iter) {
                    Ok(val) => RootsResult::Ok { value: val, method: "Native Newton-Raphson-Engine" },
                    Err(err) => RootsResult::Err { reason: err },
                }
            } else {
                RootsResult::Err { reason: "Newton-Raphson erfordert zwingend eine analytische Ableitungsfunktion f_prime." }
            }
        },
        SolverStrategy::Toms748 => {
            match toms748_root(&|x| integrand.eval(x), start, end, tol_fine) {
                Ok(val) => RootsResult::Ok { value: val, method: "SciRust TOMS 748 Engine" },
                Err(err) => RootsResult::Err { reason: err },
            }
        },
        SolverStrategy::Symbolic => {
            match symbolic_root(expression) {
                Ok(val) => RootsResult::Ok { value: val, method: "SciRust Analytical CAS Roots Engine" },
                Err(err) => RootsResult::Err { reason: err },
            }
        }
    }
}

pub fn find_root<T, F>(integrand: F, start: T, end: T) -> RootsResult<T>
where
    T: Float + FromPrimitive,
    F: RootFunction<T>,
{
    let no_derivative: Option<fn(T) -> T> = None;
    find_root_with_strategy(integrand, no_derivative, "", start, end, SolverStrategy::Auto)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_roots_engines_comparison() {
        // Thermodynamischer Härtetest: f(x) = x^2 - 4. Nullstelle liegt exakt bei x = 2.0
        let f = |x: f64| x * x - 4.0;
        let f_prime = |x: f64| 2.0 * x;

        println!("\n====================================================");
        println!(" 🔬 SCIRUST ROOTS SUITE - ALL ENGINES COMPARISON     ");
        println!("====================================================");
        println!("Gleichung: f(x) = x^2 - 4  auf Intervall [0.0, 5.0]");
        println!("----------------------------------------------------");

        // 1) Test mit der intelligenten Auto-Automatik
        match find_root_with_strategy(f, Some(f_prime), "", 0.0, 5.0, SolverStrategy::Auto) {
            RootsResult::Ok { value, method } => {
                println!("1) Strategy::Auto:");
                println!("   -> Nullstelle: {:.10}", value);
                println!("   -> Engine:     {}", method);
                assert!((value - 2.0).abs() <= 1e-6);
            },
            RootsResult::Err { reason } => panic!("Auto fehlgeschlagen: {}", reason),
        }
        println!("----------------------------------------------------");

        // 2) Test mit der reinen Brent-Engine
        match find_root_with_strategy(f, Some(f_prime), "", 0.0, 5.0, SolverStrategy::Brent) {
            RootsResult::Ok { value, method } => {
                println!("2) Strategy::Brent:");
                println!("   -> Nullstelle: {:.10}", value);
                println!("   -> Engine:     {}", method);
                assert!((value - 2.0).abs() <= 1e-6);
            },
            RootsResult::Err { reason } => panic!("Brent fehlgeschlagen: {}", reason),
        }
        println!("----------------------------------------------------");

        // 3) Test mit der reinen Newton-Raphson-Engine
        match find_root_with_strategy(f, Some(f_prime), "", 0.0, 5.0, SolverStrategy::NewtonRaphson) {
            RootsResult::Ok { value, method } => {
                println!("3) Strategy::NewtonRaphson:");
                println!("   -> Nullstelle: {:.10}", value);
                println!("   -> Engine:     {}", method);
                assert!((value - 2.0).abs() <= 1e-6);
            },
            RootsResult::Err { reason } => panic!("Newton-Raphson fehlgeschlagen: {}", reason),
        }
        println!("----------------------------------------------------");

        // 4) Test des TOMS 748 Platzhalters (Erwartet NOK / Err)
        let res_toms = find_root_with_strategy(f, Some(f_prime), "", 0.0, 5.0, SolverStrategy::Toms748);
        println!("4) Strategy::Toms748:");
        match res_toms {
            RootsResult::Ok { .. } => panic!("TOMS 748 sollte als Platzhalter fehlschlagen!"),
            RootsResult::Err { reason } => println!("   -> Status:     NOK (Vorbereitet)\n   -> Log-Info:   {}", reason),
        }
        println!("----------------------------------------------------");

        // 5) Test des Symbolischen CAS Platzhalters (Erwartet NOK / Err)
        let res_sym = find_root_with_strategy(f, Some(f_prime), "x^2-4=0", 0.0, 5.0, SolverStrategy::Symbolic);
        println!("5) Strategy::Symbolic (CAS):");
        match res_sym {
            RootsResult::Ok { .. } => panic!("Symbolic CAS sollte als Platzhalter fehlschlagen!"),
            RootsResult::Err { reason } => println!("   -> Status:     NOK (Vorbereitet)\n   -> Log-Info:   {}", reason),
        }
        println!("====================================================");
    }
}
