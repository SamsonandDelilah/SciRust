# SciRust 🚀

[![Rust Edition 2024](https://shields.io)](https://rust-lang.org)
[![License: MIT](https://shields.io)](LICENSE)

**SciRust** is a high-performance, cross-domain, and mathematically precise framework for **STEM/MINT engineering**, built natively on modern Rust architecture. It is designed to safely transform computationally heavy, unstructured legacy codebases (e.g., from Python, MATLAB, or SciLab) into a type-safe, thread-safe, and lightning-fast **Single Source of Truth (SSOT)** ecosystem.

---

## 🎯 Our Mission: The Unified STEM Platform for Engineers

In modern software engineering for physical sciences (thermodynamics, chemical kinetics, fluid dynamics), developers often lose themselves in hundreds of highly specialized libraries. **SciRust breaks this paradigm.**

We combine the intuitive **user feeling of Julia/MATLAB** with the **memory safety and uncompromising LLVM performance of Rust**. Instead of forcing you to manually configure mathematical solvers and error thresholds, SciRust utilizes intelligent **meta-interfaces (poly-algorithms)**. The system autonomously analyzes your data boundaries and selects the optimal numerical engine at compile time.

---

## 🏛️ Architecture & Core Domains

SciRust is organized as a modular **Cargo Workspace (Edition 2024)**. Every mathematical engine file (`methods.rs`) implements pure, parameterless formulas, while the interface layer (`lib.rs`) handles functional APIs, safety rails, and strategies.

### 1. 📅 `core_dates` (Time Economics & Astronomy)
*   **Focus**: Mathematically exact astronomical time tracking and Austrian financial calendar systems.
*   **Features**: Integrated Gauss Easter algorithm for dynamic holiday mapping, complete naming-collision protection for international stock exchange hours using the `_at` suffix convention. Secures the temporal baseline for brokerage engines and transaction-heavy simulations.

### 2. 📐 `mathematics::numerics::integral` (Parallel Quadrature Platform)
*   **Focus**: High-performance integration of continuous functions and discrete laboratory datasets.
*   **Features**: 
    *   **Auto Ensemble**: Automatically couples adaptive Gauss-Kronrod quadrature (G7/K15, SciPy equivalent) with a powerful *Double Exponential (Tanh-Sinh) Transformation* (`ee_integral`). 
    *   **Singularity Protection**: Mathematical poles (division-by-zero at boundaries) are automatically eliminated via *Open-Interval Mapping*.
    *   **Vector Engine**: Detects the equidistance of raw laboratory data in $O(N)$ and switches autonomously between precise Composite Simpson rules or robust Trapezoidal summation.

### 3. 🔍 `mathematics::numerics::roots` (Nonlinear Root-Finding Platform)
*   **Focus**: Indestructible bracketing and high-speed solving of single-variable equations $f(x) = 0$.
*   **Features**: 
    *   **Two-Stage Rocket**: Launches with an optimized Brent method for absolute bracketing safety up to a coarse threshold of `1e-4`, then seamlessly fires a highly precise *Quasi-Newton (Secant) final push* up to `1e-12`—hitting exact physical zeros without requiring user-provided analytical derivatives.
    *   **CAS Infrastructure**: Pre-configured API gates ready for future symbolic solvers and analytical term resolution.

### 4. 🧪 `physics::thermodynamics` (Equations of State / EoS)
*   **Focus**: Native implementation of real thermodynamic equations of state on an industrial scale.
*   **Features**: Purely generic models for *Ideal Gas*, *Van der Waals (vdW)*, *Redlich-Kwong (RK)*, *Soave-Redlich-Kwong (SRK)*, *Peng-Robinson (PR)*, as well as general *Cubic EoS (C3)* and *Virial Equations*. Deeply integrated with the `roots` package to calculate precise molar volumes $v$ for any target pressure.

---

## ⚡ Performance Benchmark: What the Code Can Do

SciRust maximizes the power of the Rust type system using **Generics and Trait Bounds** (`num-traits`). All mathematical operations utilize *Static Dispatch (Monomorphization)*, forcing the compiler to generate raw machine code tailored to your exact primitive (`f32`, `f64`, or arbitrary precision)—**completely free of runtime overhead**.

Our internal benchmark (`bench_app`) for improper integrals with boundary singularities demonstrates the following performance footprint on modern CPUs:

*   **SciRust ee_integral**: **~6.5 µs** (Deviation: $1.29 \times 10^{-8}$) -> **World Class** 🏆
*   **SciRust Auto Ensemble**: **~7.7 µs** (Including full autonomous engine safety)
*   **Established External Crates**: **~32.7 µs** (Struggling at the mathematical boundary)

---

## 🛠️ Getting Started for Developers

To compile the entire workspace, run the comprehensive test suites, or execute the performance benchmark on your machine, clone the repository and use standard Cargo commands:

```bash
# Build the entire workspace with full optimization
cargo build --release

# Execute the cross-domain automated test suite
cargo test

# Launch the integrated benchmark suite
cargo run --release -p bench_app
```

---

## 💡 Engineering Roadmap (The Horizon)

While our numerical integration and root-finding baseline is solid, our path toward becoming the leading open-source STEM platform has just begun:
*   [ ] **`differentiation`**: Numerical differentiation matrices to calculate exact Jacobian and Hessian systems.
*   [ ] **Multivariate Systems**: Multi-dimensional nonlinear equations (Modified Powell Hybrid Method / Levenberg-Marquardt) to solve complex chemical equilibria.
*   [ ] **Symbolic CAS (`python-cas`)**: Optional `pyo3` bindings to SymPy for speculative, parallel symbolic differentiation "into the blue."

---

## 💬 Join the Lab: Head Over to Discussions!

This project thrives on the intersection of hardcore numerical engineering and systems programming expertise. If you have ideas for new thermodynamic parameters, want to submit a solver, or want to discuss edge-case mathematical profiles:

**Our GitHub Discussions are fully activated and open for business!**

👉 **[Jump straight into the SciRust Discussions]**

We look forward to your insights in the following channels:
*   💡 **Ideas**: Which physical models or numerical solvers should we integrate next?
*   🔧 **Engineering Practice**: Real-world reports—where do your legacy Python/MATLAB scripts currently hit performance walls?
*   🦀 **Rust Pro-Tips**: Optimizations for SIMD compilation, low-level vectorization, or compiler-guided type inference.

---

## ⚖️ License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.