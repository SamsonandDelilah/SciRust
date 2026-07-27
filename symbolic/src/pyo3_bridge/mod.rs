// symbolic/src/pyo3_bridge/mod.rs

use pyo3::prelude::*;

/// Initialize the Python environment and GIL context.
pub fn init_python_context() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;
        println!("Python Environment Initialized: {}", version);
        Ok(())
    })
}

/// Take a raw Python/SymPy expression string and return the processed SymPy expression string.
pub fn evaluate_sympy_expression(expr_str: &str, x_val: f64) -> PyResult<f64> {
    Python::with_gil(|py| {
        let sympy = py.import("sympy")?;
        let x = sympy.call_method1("Symbol", ("x",))?;
        
        // Parse string expression into SymPy expression
        let locals = pyo3::types::PyDict::new(py);
        locals.set_item("x", x)?;
        let expr = sympy.call_method("sympify", (expr_str,), Some(&locals))?;
        
        // Substitute value and evaluate numerically
        let substituted = expr.call_method1("subs", (("x", x_val),))?;
        let result: f64 = substituted.call_method0("evalf")?.extract()?;
        
        Ok(result)
    })
}