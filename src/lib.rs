pub extern crate pyo3;
pub extern crate built;

#[macro_export]
macro_rules! pyo3_built {
    ($py: ident, $info: ident) => {{

        use ::pyo3::types::PyDict;
        use ::pyo3::types::PyString;
        use $crate::built::util::strptime;

        let info = PyDict::new($py);

        // Rustc
        let build = PyDict::new($py);
        build.set_item("rustc", $info::RUSTC)?;
        build.set_item("rustc-version", $info::RUSTC_VERSION)?;
        build.set_item("opt-level", $info::OPT_LEVEL)?;
        build.set_item("debug", $info::DEBUG)?;
        build.set_item("jobs", $info::NUM_JOBS)?;
        info.set_item("build", build)?;

        // info time
        let ts = strptime($info::BUILT_TIME_UTC).timestamp();
        let dt = $py
            .import("datetime")?
            .get("datetime")?
            .to_object($py)
            .call_method1($py, "fromtimestamp", (ts,))?;
        info.set_item("info-time", dt)?;

        // info dependencies
        let deps = PyDict::new($py);
        for (name, version) in $info::DEPENDENCIES.iter() {
            deps.set_item(name, version)?;
        }
        info.set_item("dependencies", deps)?;

        // Features
        let features = $info::FEATURES
            .iter()
            .map(|feat| PyString::new($py, feat)).collect::<Vec<_>>();
        info.set_item("features", features)?;

        // Host
        let host = PyDict::new($py);
        host.set_item("triple", $info::HOST)?;
        info.set_item("host", host)?;

        // Target
        let target = PyDict::new($py);
        target.set_item("arch", $info::CFG_TARGET_ARCH)?;
        target.set_item("os", $info::CFG_OS)?;
        target.set_item("family", $info::CFG_FAMILY)?;
        target.set_item("env", $info::CFG_ENV)?;
        target.set_item("triple", $info::TARGET)?;
        target.set_item("endianness", $info::CFG_ENDIAN)?;
        target.set_item("pointer-width", $info::CFG_POINTER_WIDTH)?;
        target.set_item("profile", $info::PROFILE)?;
        info.set_item("target", target)?;

        info
    }};
}
