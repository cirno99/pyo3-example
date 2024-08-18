use pyo3::prelude::*;

#[pyfunction]
fn check_limit(limit: i32) -> bool {
    let valid_range = 1..101;
    valid_range.contains(&limit)
}

/// A Python module implemented in Rust.
#[pymodule]
fn ejemplo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_limit, m)?)?;
    Ok(())
}
