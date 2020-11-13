//! cfg:
//!
//!

use phper_sys::{PHP_DEBUG, PHP_MAJOR_VERSION, PHP_MINOR_VERSION, PHP_RELEASE_VERSION, USING_ZTS};

/// Register useful rust cfg for project using phper.
pub fn register_configures() {
    // versions
    println!(
        "cargo:rustc-cfg=phper_major_version=\"{}\"",
        PHP_MAJOR_VERSION
    );
    println!(
        "cargo:rustc-cfg=phper_minor_version=\"{}\"",
        PHP_MINOR_VERSION
    );
    println!(
        "cargo:rustc-cfg=phper_release_version=\"{}\"",
        PHP_RELEASE_VERSION
    );
    println!(
        "cargo:rustc-cfg=phper_php_version=\"{}.{}\"",
        PHP_MAJOR_VERSION, PHP_MINOR_VERSION,
    );

    if PHP_DEBUG > 0 {
        println!("cargo:rustc-cfg=phper_debug");
    }

    if USING_ZTS > 0 {
        println!("cargo:rustc-cfg=phper_zts");
    }
}
