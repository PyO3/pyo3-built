#[macro_use]
extern crate pyo3_built;
extern crate pyo3;

use pyo3::prelude::*;

#[allow(dead_code)]
mod build {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[pyfunction]
fn hello() {
    println!("Hello, world!");
}

/// Module documentation string
#[pymodule]
#[pyo3(name = "hello")]
fn init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__build__", pyo3_built!(py, build))?;
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
