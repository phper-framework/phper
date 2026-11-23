use phper_test::{cargo::CargoBuilder, cli::test_php_script, log};
use std::{
    env,
    path::{Path, PathBuf},
    sync::LazyLock,
};

pub static DYLIB_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    log::setup();
    let result = CargoBuilder::new()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .build()
        .unwrap();
    result.get_cdylib().unwrap()
});

pub static TESTS_PHP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("php")
});

#[test]
fn test_php() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("test.php"));
}
