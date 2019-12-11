use bindgen::Builder;
use std::env;
use std::error::Error;
use std::path::{PathBuf, Path};
use std::process::Command;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    if !Path::new("php-src/Zend/zend_config.h").exists() {
        Command::new("sh")
            .args(&["-c", "cd php-src && ./buildconf --force && ./configure"])
            .status()?;
    }

    let bindings = Builder::default()
        .header("php-src/main/php.h")
        .clang_args(&[
            "-Iphp-src",
            "-Iphp-src/main",
            "-Iphp-src/Zend",
            "-Iphp-src/TSRM",
        ])
        .blacklist_item("FP_INT_UPWARD")
        .blacklist_item("FP_INT_TONEARESTFROMZERO")
        .blacklist_item("FP_NAN")
        .blacklist_item("FP_INFINITE")
        .blacklist_item("FP_ZERO")
        .blacklist_item("FP_NORMAL")
        .blacklist_item("FP_INT_DOWNWARD")
        .blacklist_item("FP_INT_TOWARDZERO")
        .blacklist_item("FP_INT_TONEAREST")
        .blacklist_item("FP_SUBNORMAL")
        .blacklist_type("timex")
        .blacklist_function("clock_adjtime")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("php_bindings.rs"))?;

    Ok(())
}
