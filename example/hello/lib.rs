#[macro_use]
extern crate pyo3_built;
extern crate pyo3;

use pyo3::prelude::*;

#[allow(dead_code)]
mod build {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// Module documentation string
#[pymodule]
#[pyo3(name = "hello")]
fn init(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m)]
    fn hello(_py: Python) -> PyResult<()> {
        println!("Hello, world!");
        Ok(())
    }

    m.add("__build__", pyo3_built!(py, build))?;
    Ok(())
}
