use integral::{integrate_with_strategy, integrate_vectors, SolverStrategy, IntegrationResult};
use differentiation::{differentiate_with_strategy, DifferentiationStrategy, DifferentiationResult};
use std::time::Instant;

fn main() {
    let integrand = |x: f64| {
        if x <= 1e-12 { 0.0f64 } else { (-x / 5.0).exp() * x.powf(-1.0 / 3.0) }
    };
    
    let target_value: f64 = 3.6798142583691758;
    let iterations = 10_000; 

    println!("=====================================================================");
    println!(" 🚀 SCIRUST EXTENDED BENCHMARK - ALL LÖSER COMPETE                  ");
    println!("=====================================================================");
    println!("Integrand:  f(x) = exp(-x/5) * x^(-1/3) auf [0.0, 10.0]");
    println!("Durchläufe: {} serielle Berechnungen pro Engine", iterations);
    println!("---------------------------------------------------------------------");

    // 1. BENCHMARK: SciRust Auto
    let start = Instant::now();
    let mut res_auto = 0.0;
    for _ in 0..iterations {
        if let IntegrationResult::Ok { value, .. } = integrate_with_strategy(integrand, "", 0.0, 10.0, SolverStrategy::Auto) { res_auto = value; }
    }
    let dur_auto = start.elapsed();
    println!("1) SciRust AUTO ENSEMBLE:        {:.10} | Abw: {:.10} | {:3.3} µs", res_auto, (res_auto - target_value).abs(), dur_auto.as_micros() as f64 / iterations as f64);

    // 2. BENCHMARK: SciRust ee_integral
    let start = Instant::now();
    let mut res_ee = 0.0;
    for _ in 0..iterations {
        if let IntegrationResult::Ok { value, .. } = integrate_with_strategy(integrand, "", 0.0, 10.0, SolverStrategy::DoubleExponential) { res_ee = value; }
    }
    let dur_ee = start.elapsed();
    println!("2) SciRust ee_integral:          {:.10} | Abw: {:.10} | {:3.3} µs", res_ee, (res_ee - target_value).abs(), dur_ee.as_micros() as f64 / iterations as f64);

    // 3. BENCHMARK: SciRust Romberg
    let start = Instant::now();
    let mut res_rom = 0.0;
    for _ in 0..iterations {
        if let IntegrationResult::Ok { value, .. } = integrate_with_strategy(integrand, "", 0.0, 10.0, SolverStrategy::Romberg) { res_rom = value; }
    }
    let dur_rom = start.elapsed();
    println!("3) SciRust Romberg:              {:.10} | Abw: {:.10} | {:3.3} µs", res_rom, (res_rom - target_value).abs(), dur_rom.as_micros() as f64 / iterations as f64);

    // 4. BENCHMARK: SciRust Simpson (Quick Estimation)
    let start = Instant::now();
    let mut res_simp = 0.0;
    for _ in 0..iterations {
        if let IntegrationResult::Ok { value, .. } = integrate_with_strategy(integrand, "", 0.0, 10.0, SolverStrategy::Simpson) { res_simp = value; }
    }
    let dur_simp = start.elapsed();
    println!("4) SciRust Simpson (100 Steps):  {:.10} | Abw: {:.10} | {:3.3} µs", res_simp, (res_simp - target_value).abs(), dur_simp.as_micros() as f64 / iterations as f64);

    // 5. BENCHMARK: SciRust Trapezoidal (Quick Estimation)
    let start = Instant::now();
    let mut res_trap = 0.0;
    for _ in 0..iterations {
        if let IntegrationResult::Ok { value, .. } = integrate_with_strategy(integrand, "", 0.0, 10.0, SolverStrategy::Trapezoidal) { res_trap = value; }
    }
    let dur_trap = start.elapsed();
    println!("5) SciRust Trapez (100 Steps):   {:.10} | Abw: {:.10} | {:3.3} µs", res_trap, (res_trap - target_value).abs(), dur_trap.as_micros() as f64 / iterations as f64);

    println!("---------------------------------------------------------------------");
    println!(" 📊 PRÜFUNG DER DISKRETEN VEKTOR-ENGINE (LABORDATEN)                 ");
    println!("---------------------------------------------------------------------");
    // Wir simulieren eine Messreihe mit 5 äquidistanten Punkten für f(x) = x^2 von 0 bis 2 (Zielwert: 2.66666)
    let x_data = vec![0.0, 0.5, 1.0, 1.5, 2.0];
    let y_data = vec![0.0, 0.25, 1.0, 2.25, 4.0];
    
    if let IntegrationResult::Ok { value, method } = integrate_vectors(&x_data, &y_data) {
        println!("Vektor-Integral:  {:.6} (Erwartet bei Simpson: 2.666667)", value);
        println!("Gewählte Engine:  {}", method);
    }
    println!("=====================================================================");

    // Beispiel: Ableitung von f(x) = x^3 bei x = 2.0 (Analytisch: 3*x^2 = 12.0)
    let f = |x: f64| x.powi(3);
    
    let res = differentiate_with_strategy(
        f, 
        2.0, 
        DifferentiationStrategy::HighOrder5, 
        None
    );

    match res {
        DifferentiationResult::Ok { value, error_estimate, method } => {
            println!("Ergebnis: {} (Methode: {}, Fehler: {})", value, method, error_estimate);
        }
        DifferentiationResult::Err { reason } => {
            println!("Fehler: {}", reason);
        }
    }
}