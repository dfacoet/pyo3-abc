use pyo3::prelude::*;
mod container;
use crate::container::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_abc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<RustSet>()?;
    Ok(())
}

#[pyclass]
pub struct RustSet {
    values: std::collections::HashSet<i32>,
}

#[pymethods]
impl Container<i32> for RustSet {
    fn __contains__(&self, item: &i32) -> bool {
        self.values.contains(item)
    }
}
