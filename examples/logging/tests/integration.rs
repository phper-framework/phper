// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper_test::{cargo::CargoBuilder, cli::test_php_script_with_condition, log};
use std::{
    env,
    path::{Path, PathBuf},
    str,
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
fn test_php_say() {
    test_php_script_with_condition(
        &*DYLIB_PATH,
        TESTS_PHP_DIR.join("test_php_say.php"),
        |output| {
            let stdout = str::from_utf8(&output.stdout).unwrap();
            stdout == "Hello, world!" && output.status.success()
        },
    );
}

#[test]
fn test_php_notice() {
    test_php_script_with_condition(
        &*DYLIB_PATH,
        TESTS_PHP_DIR.join("test_php_notice.php"),
        |output| {
            let stdout = str::from_utf8(&output.stdout).unwrap();
            stdout.contains("Notice:")
                && stdout.contains("Something happened: just for test")
                && output.status.success()
        },
    );
}

#[test]
fn test_php_warning() {
    test_php_script_with_condition(
        &*DYLIB_PATH,
        TESTS_PHP_DIR.join("test_php_warning.php"),
        |output| {
            let stdout = str::from_utf8(&output.stdout).unwrap();
            stdout.contains("Warning:")
                && stdout.contains("Something warning: just for test")
                && output.status.success()
        },
    );
}

#[test]
fn test_php_error() {
    test_php_script_with_condition(
        &*DYLIB_PATH,
        TESTS_PHP_DIR.join("test_php_error.php"),
        |output| {
            let stdout = str::from_utf8(&output.stdout).unwrap();
            stdout.contains("Fatal error:")
                && stdout.contains("Something gone failed: just for test")
        },
    );
}

#[test]
fn test_php_deprecated() {
    test_php_script_with_condition(
        &*DYLIB_PATH,
        TESTS_PHP_DIR.join("test_php_deprecated.php"),
        |output| {
            let stdout = str::from_utf8(&output.stdout).unwrap();
            stdout.contains("Deprecated:")
                && stdout.contains("Something deprecated: just for test")
                && output.status.success()
        },
    );
}
