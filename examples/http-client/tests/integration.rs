// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

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
    use std::{process::Command, thread::sleep, time::Duration};

    let router = TESTS_PHP_DIR.join("router.php");
    let server = Command::new("php")
        .arg("-S")
        .arg("127.0.0.1:8000")
        .arg("-t")
        .arg(TESTS_PHP_DIR.to_str().unwrap())
        .arg(router.to_str().unwrap())
        .spawn()
        .expect("Failed to start PHP built-in server");

    struct ServerGuard(std::process::Child);
    impl Drop for ServerGuard {
        fn drop(&mut self) {
            let _ = self.0.kill();
        }
    }
    let _guard = ServerGuard(server);

    // Give the server time to start
    sleep(Duration::from_secs(1));

    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("test.php"));
}
