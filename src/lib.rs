#![doc = include_str!("../README.md")]

#[macro_export]
macro_rules! pyo3_built {
    ($py: ident, $info: ident, $dict: ident, "build") => {
        // Rustc
        let build = PyDict::new_bound($py);
        build.set_item("rustc", $info::RUSTC)?;
        build.set_item("rustc-version", $info::RUSTC_VERSION)?;
        build.set_item("opt-level", $info::OPT_LEVEL)?;
        build.set_item("debug", $info::DEBUG)?;
        build.set_item("jobs", $info::NUM_JOBS)?;
        $dict.set_item("build", build)?;
    };
    ($py: ident, $info: ident, $dict: ident, "time") => {
        let dt = $py
            .import_bound("email.utils")?
            .call_method1("parsedate_to_datetime", ($info::BUILT_TIME_UTC,))?;
        $dict.set_item("info-time", dt)?;
    };
    ($py: ident, $info: ident, $dict: ident, "deps") => {
        // info dependencies
        let deps = PyDict::new_bound($py);
        for (name, version) in $info::DEPENDENCIES.iter() {
            deps.set_item(name, version)?;
        }
        $dict.set_item("dependencies", deps)?;
    };
    ($py: ident, $info: ident, $dict: ident, "features") => {
        // Features
        let features = $info::FEATURES
            .iter()
            .map(|feat| PyString::new_bound($py, feat))
            .collect::<Vec<_>>();
        $dict.set_item("features", features)?;
    };
    ($py: ident, $info: ident, $dict: ident, "host") => {
        // Host
        let host = PyDict::new_bound($py);
        host.set_item("triple", $info::HOST)?;
        $dict.set_item("host", host)?;
    };
    ($py: ident, $info: ident, $dict: ident, "target") => {
        // Target
        let target = PyDict::new_bound($py);
        target.set_item("arch", $info::CFG_TARGET_ARCH)?;
        target.set_item("os", $info::CFG_OS)?;
        target.set_item("family", $info::CFG_FAMILY)?;
        target.set_item("env", $info::CFG_ENV)?;
        target.set_item("triple", $info::TARGET)?;
        target.set_item("endianness", $info::CFG_ENDIAN)?;
        target.set_item("pointer-width", $info::CFG_POINTER_WIDTH)?;
        target.set_item("profile", $info::PROFILE)?;
        $dict.set_item("target", target)?;
    };
    ($py: ident, $info: ident, $dict: ident, "git") => {
        let git = PyDict::new_bound($py);
        git.set_item("version", $info::GIT_VERSION)?;
        git.set_item("dirty", $info::GIT_DIRTY)?;
        git.set_item("hash", $info::GIT_COMMIT_HASH)?;
        git.set_item("head", $info::GIT_HEAD_REF)?;
        $dict.set_item("git", git)?;

    };
    // Default build
    ($py: ident, $info: ident) => {
        pyo3_built!{$py, $info, "build", "time", "deps", "features", "host", "target"}
    };
    // Custom build
    ($py: ident, $info: ident, $($i:tt ),+ ) => {{
        use pyo3::types::PyDict;
        use pyo3::types::PyString;
        let info = PyDict::new_bound($py);
        $(
            pyo3_built!{$py,$info, info, $i}
        )+
        info
    }};
}
