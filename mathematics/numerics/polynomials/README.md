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

## Todo / In Progress

* **Estrin's Scheme (`EvaluationStrategy::Estrin`):**
* Designed to maximize parallel instruction-level execution, currently prepared in the architecture but temporarily disabled in the dispatch enum pending final integration.


* **Lagrange Interpolation:**
* Planned support for constructing polynomials directly from a set of interpolation nodes.



---

## Roadmap & Outlook

* **GPU Acceleration:** Offloading intensive numerical loops to graphics processing units for massively parallel scientific computations.
* **Advanced Numerical Methods:** Integration of automatic error bounding and support for additional polynomial bases (e.g., Legendre polynomials).