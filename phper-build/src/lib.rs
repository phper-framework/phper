#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]

/*!
Generate stubs for project using [phper](https://crates.io/crates/phper).

Add this crate in your `[build-dependencies]` and using in `build.rs`.

## License

[Unlicense](https://github.com/jmjoy/phper/blob/master/LICENSE).
*/

use phper_sys::*;

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
