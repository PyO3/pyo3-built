# `pyo3-built`

*Simple macro to expose metadata obtained with the [`built`](https://crates.io/crates/built)
crate as a [`PyDict`](https://pyo3.github.io/pyo3/pyo3/struct.PyDict.html)*


## Usage

Add the following to your `Cargo.toml` manifest:
```toml
[build-dependencies]
built = "^0.3"

[dependencies]
pyo3-built = "^0.1"
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
    built::write_built_file_with_opts(&opts, src, dst)
        .expect("Failed to acquire build-time information");
}
```

Then, include the generated file anywhere in a dedicated module in your Python
extension:
```rust
//! lib.rs
#![feature(proc_macro)]

#[macro_use]
extern crate pyo3_built;
extern crate pyo3;

use pyo3::prelude::*;
use pyo3::py::modinit;

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
      "pyo3": "0.2.6",
      "pyo3-built": "0.1.0",
      "pyo3cls": "0.2.1",
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
