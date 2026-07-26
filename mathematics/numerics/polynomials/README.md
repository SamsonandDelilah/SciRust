# Polynomials Library

A highly optimized Rust library for efficient polynomial computation, numerical evaluation, and symbolic manipulation, supporting both scalar and hardware-accelerated SIMD (AVX2/FMA) execution paths.

---

## Features Provided

* **Flexible Polynomial Structure (`Polynomial<T>`):**
  * Generic support for floating-point types (`f32`, `f64`, etc.).
  * Standard coefficient convention ($c_0$ as the constant term up to $c_n$).
* **Evaluation Strategies:**
  * **Horner's Scheme:** Numerically stable and performant scalar evaluation.
  * **Optimized SIMD Acceleration (AVX2 & FMA):** Hardware-accelerated vectorization for 4-wide (`f64`) and 8-wide (`f32`) parallel processing, fully compatible with both even and odd polynomial degrees.
  * **Chebyshev Evaluation:** Specialized evaluation routines for Chebyshev expansions.
* **Analysis & Manipulation:**
  * Efficient derivative calculation (`derivative()`).
  * Strategy-based dispatch framework (`evaluate_with_strategy`).

---

## Usage Examples

```rust
use mathematics::numerics::polynomials::{Polynomial, evaluate_horner, evaluate_chebyshev}; // Pfad je nach Projektstruktur anpassen

fn main() {
    // 1. Using the Polynomial struct with Horner's scheme
    let poly = Polynomial::new(vec![1.0, 2.0, 3.0]); // p(x) = 1.0 + 2.0*x + 3.0*x^2
    let result_poly = poly.evaluate(2.0);
    println!("Polynomial evaluate result: {}", result_poly); // Output: 17.0

    // 2. Direct function call for Horner's scheme
    let coefficients = vec![1.0, 2.0, 3.0];
    let x = 2.0;
    let result_horner = evaluate_horner(&coefficients, x);
    println!("Direct Horner result: {}", result_horner); // Output: 17.0

    // 3. Direct function call for Chebyshev evaluation
    let cheb_coeffs = vec![1.0, 0.5, 0.1];
    let result_chebyshev = evaluate_chebyshev(&cheb_coeffs, x);
    println!("Direct Chebyshev result: {}", result_chebyshev);
}