#![feature(proc_macro)]

#[macro_use]
extern crate pyo3_built;
extern crate pyo3;

use pyo3::prelude::*;
use pyo3::py::modinit;

mod build {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// Module documentation string
#[modinit("hello")]
fn init(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "hello")]
    fn hello(_py: Python) -> PyResult<()> {
        println!("Hello, world!");
        Ok(())
    }

    m.add("__build__", pyo3_built!(py, build))?;
    Ok(())
}
