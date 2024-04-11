# `pyo3-built`

*Simple macro to expose metadata obtained with the [`built`](https://crates.io/crates/built)
crate as a [`PyDict`](https://pyo3.github.io/pyo3/pyo3/struct.PyDict.html)*

[![Build](https://img.shields.io/github/actions/workflow/status/PyO3/pyo3-built/test.yml?branch=main&maxAge=600&style=flat-square)](https://github.com/PyO3/pyo3-built/actions)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/apache-2.0/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/PyO3/pyo3-built)
[![Crate](https://img.shields.io/crates/v/pyo3-built.svg?maxAge=600&style=flat-square)](https://crates.io/crates/pyo3-built)
[![GitHub issues](https://img.shields.io/github/issues/PyO3/pyo3-built.svg?style=flat-square)](https://github.com/PyO3/pyo3-built/issues)

## Usage

Add the following to your `Cargo.toml` manifest:
```toml
[build-dependencies]
built = { version = "0.7", features = ["chrono"] }
[dependencies]
pyo3-built = "0.5"
```

Create your `build.rs` as you normally would with `built`, but activate
dependencies metadata as well:
```rust,ignore
//! build.rs
extern crate built;

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");
}
```

Then, include the generated file anywhere in a dedicated module in your Python
extension, and use the `pyo3_built!` macro to generate the `PyDict`:
```rust,ignore
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

### Customization

When invoking the macro, one can control what will be added
to the build dictionary by postfixing the list of the parameters we want in the dictionary.
See the following example:
```rust,ignore
m.add("__build__", pyo3_built!(py, build, "time", "git", "target"))?;
```

The following parameters are available, mirroring built categories:

- `"build"`
- `"time"` (requires the `chrono` feature of `built`)
- `"deps"`
- `"features"` (requires the `cargo-lock` feature of `built`)
- `"host"`
- `"target"`
- `"git"` (requires the `git2` feature of `built`)

By default everything except `"git"` is enabled.
