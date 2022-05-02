#!/usr/bin/env python
# coding: utf-8
import setuptools
import setuptools_rust as rust

setuptools.setup(
    name="hello",
    author="Martin Larralde",
    author_email="martin.larralde@ens-cachan.fr",
    description="Example Python module using pyo3-built",
    setup_requires=[
        "setuptools",
        "setuptools-rust ~=0.9",
    ],
    rust_extensions=[
        rust.RustExtension(
            "hello",
            "hello/Cargo.toml",
            binding=rust.Binding.PyO3,
            strip=rust.Strip.Debug,
        )
    ]
)
