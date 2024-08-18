pub mod prime;

use prime::prime;
use pyo3::prelude::*;

#[pyfunction]
fn check_limit(limit: i32) -> bool {
    let valid_range = 1..101;
    valid_range.contains(&limit)
}

#[pyfunction]
fn test_prime(num: i32) -> bool {
    for v in prime(500) {
        println!("p = {}", v);
    }
    true
}

/// A Python module implemented in Rust.
#[pymodule]
fn ejemplo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_limit, m)?)?;
    m.add_function(wrap_pyfunction!(test_prime, m)?)?;
    Ok(())
}
