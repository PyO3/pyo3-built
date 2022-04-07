#[macro_export]
macro_rules! pyo3_built {
    ($py: ident, $info: ident, $dict: ident, "build") => {
        // Rustc
        let build = PyDict::new($py);
        build.set_item("rustc", $info::RUSTC)?;
        build.set_item("rustc-version", $info::RUSTC_VERSION)?;
        build.set_item("opt-level", $info::OPT_LEVEL)?;
        build.set_item("debug", $info::DEBUG)?;
        build.set_item("jobs", $info::NUM_JOBS)?;
        $dict.set_item("build", build)?;
    };
    ($py: ident, $info: ident, $dict: ident, "time") => {
        // info time
        let dt = $py
            .import("email.utils")?
            .getattr("parsedate_to_datetime")?
            .call1(($info::BUILT_TIME_UTC,))?;
        /*let ts = strptime($info::BUILT_TIME_UTC).timestamp();
        let dt = $py
            .import("datetime")?
            .get("datetime")?
            .to_object($py)
            .call_method1($py, "fromtimestamp", (ts,))?;*/
        $dict.set_item("info-time", dt)?;
    };
    ($py: ident, $info: ident, $dict: ident, "deps") => {
        // info dependencies
        let deps = PyDict::new($py);
        for (name, version) in $info::DEPENDENCIES.iter() {
            deps.set_item(name, version)?;
        }
        $dict.set_item("dependencies", deps)?;
    };
    ($py: ident, $info: ident, $dict: ident, "features") => {
        // Features
        let features = $info::FEATURES
            .iter()
            .map(|feat| PyString::new($py, feat))
            .collect::<Vec<_>>();
        $dict.set_item("features", features)?;
    };
    ($py: ident, $info: ident, $dict: ident, "host") => {
        // Host
        let host = PyDict::new($py);
        host.set_item("triple", $info::HOST)?;
        $dict.set_item("host", host)?;
    };
    ($py: ident, $info: ident, $dict: ident, "target") => {
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
        $dict.set_item("target", target)?;
    };
    ($py: ident, $info: ident, $dict: ident, "git") => {
        let git = PyDict::new($py);
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
        let info = PyDict::new($py);
        $(
            pyo3_built!{$py,$info, info, $i}
        )+
        info
    }};
}
