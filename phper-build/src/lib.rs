use phper_sys::{
    USING_ZTS,
    PHP_DEBUG,
    PHP_MAJOR_VERSION,
    PHP_MINOR_VERSION,
};

/// Register useful rust cfg for project using phper.
pub fn register_configures() {
    register_versions();

    if PHP_DEBUG > 0 {
        println!("cargo:rustc-cfg=phper_debug");
    }
    if USING_ZTS > 0 {
        println!("cargo:rustc-cfg=phper_zts");
    }
}

fn register_versions() {
}