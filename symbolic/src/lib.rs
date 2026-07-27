// 1. Native symbolische Engine (reines Rust, für die Zukunft)
//pub mod native_symbolic;

// 2. PyO3 / SymPy Brücke (nur bei aktiviertem Feature)
#[cfg(feature = "python-bindings")]
pub mod pyo3_bridge;