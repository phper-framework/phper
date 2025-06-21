use phper_test::{cargo::CargoBuilder, fpm::FpmHandle};
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

#[allow(dead_code)]
pub(crate) static DYLIB_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let result = CargoBuilder::new()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .build()
        .unwrap();
    result.get_cdylib().unwrap()
});

#[allow(dead_code)]
pub(crate) static TESTS_PHP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("php")
});

#[allow(dead_code)]
pub(crate) static FPM_HANDLE: LazyLock<&FpmHandle> =
    LazyLock::new(|| FpmHandle::setup(&*DYLIB_PATH));
