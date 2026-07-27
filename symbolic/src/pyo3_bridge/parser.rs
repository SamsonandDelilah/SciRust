// symbolic/src/pyo3_bridge/parser.rs

use pyo3::prelude::*;
use pyo3::types::PyDict;

// symbolic/src/pyo3_bridge/parser.rs

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// SymPy Expression Parser Wrapper
pub struct SymPyParser;

impl SymPyParser {
    /// Parse a SymPy-compatible expression string and return the string representation.
    pub fn parse(expr_str: &str) -> PyResult<String> {
        Python::with_gil(|py| {
            let sympy = py.import("sympy")?;
            let x = sympy.call_method1("Symbol", ("x",))?;
            
            let locals = PyDict::new(py);
            locals.set_item("x", x)?;
            
            let expr = sympy.call_method("sympify", (expr_str,), Some(&locals))?;
            let result_str: String = expr.call_method0("__str__")?.extract()?;
            Ok(result_str)
        })
    }

    /// Compute the symbolic derivative of an expression with respect to 'x' and return it as a string.
    pub fn differentiate(expr_str: &str) -> PyResult<String> {
        Python::with_gil(|py| {
            let sympy = py.import("sympy")?;
            let x = sympy.call_method1("Symbol", ("x",))?;
            
            let locals = PyDict::new(py);
            locals.set_item("x", x)?;
            
            let expr = sympy.call_method("sympify", (expr_str,), Some(&locals))?;
            let deriv = sympy.call_method1("diff", (expr, x))?;
            
            let deriv_str: String = deriv.call_method0("__str__")?.extract()?;
            Ok(deriv_str)
        })
    }
}