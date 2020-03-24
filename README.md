# `pyo3-built`

*Simple macro to expose metadata obtained with the [`built`](https://crates.io/crates/built)
crate as a [`PyDict`](https://pyo3.github.io/pyo3/pyo3/struct.PyDict.html)*

[![TravisCI](https://img.shields.io/travis/PyO3/pyo3-built/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/PyO3/pyo3-built/branches)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/apache-2.0/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/PyO3/pyo3-built)
[![Crate](https://img.shields.io/crates/v/pyo3-built.svg?maxAge=600&style=flat-square)](https://crates.io/crates/pyo3-built)
[![GitHub issues](https://img.shields.io/github/issues/PyO3/pyo3-built.svg?style=flat-square)](https://github.com/PyO3/pyo3-built/issues)

## Usage

Add the following to your `Cargo.toml` manifest:
```toml
[build-dependencies]
built = { version = "0.4", features = ["chrono"] }
[dependencies]
pyo3-built = "0.4"
```

Create your `build.rs` as you normally would with `built`, but activate
dependencies metadata as well:
```rust
//! build.rs
extern crate built;

fn main() {
    let src = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("built.rs");
    let mut opts = built::Options::default();
    opts.set_dependencies(true)
        .set_compiler(true)
        .set_env(true);
    built::write_built_file_with_opts(&opts, std::path::Path::new(&src), &dst)
        .expect("Failed to acquire build-time information");
}
```

Then, include the generated file anywhere in a dedicated module in your Python
extension, and use the `pyo3_built!` macro to generate the `PyDict`:
```rust
//! lib.rs
#[macro_use]
extern crate pyo3_built;
extern crate pyo3;

use pyo3::prelude::*;

#[allow(dead_code)]
mod build {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// Module documentation string
#[modinit("mymodule")]
fn init(py: Python, m: &PyModule) -> PyResult<()> {
    // ... //
    m.add("__build__", pyo3_built!(py, build))?;
    Ok(())
}
```

That's it ! After compiling your extension module, the `__build__` attribute
will contain the following metadata:
```python
>>> import mymodule
>>> mymodule.__build__
{
   "build-time": datetime.datetime(2018, 5, 11, 16, 43, 28),
   "debug": true,
   "dependencies": {
      ...
      "pyo3": "0.6.0",
      "pyo3-built": "0.1.0",
      "pyo3cls": "0.6.0",
      ...
   },
   "features": [
      "PYO3"
   ],
   "host": {
      "triple": "x86_64-unknown-linux-gnu"
   },
   "opt-level": "0",
   "rustc": "rustc",
   "rustc-version": "rustc 1.27.0-nightly (acd3871ba 2018-05-10)",
   "target": {
      "arch": "x86_64",
      "endianness": "little",
      "env": "gnu",
      "family": "unix",
      "os": "linux",
      "pointer-width": "64",
      "profile": "debug",
      "triple": "x86_64-unknown-linux-gnu"
   }
}
```
