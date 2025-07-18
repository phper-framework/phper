// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use axum::http::header::CONTENT_TYPE;
use hyper::StatusCode;
use phper_test::{cargo::CargoBuilder, cli::test_long_term_php_script_with_condition, log};
use reqwest::blocking::Client;
use std::{
    env,
    path::{Path, PathBuf},
    sync::LazyLock,
    thread::sleep,
    time::Duration,
};

pub static DYLIB_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    log::setup();
    let result = CargoBuilder::new()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .build()
        .unwrap();
    result.get_cdylib().unwrap()
});

#[test]
fn test_php() {
    test_long_term_php_script_with_condition(
        &*DYLIB_PATH,
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test.php"),
        |_| {
            // wait for server startup.
            sleep(Duration::from_secs(3));

            let client = Client::new();
            for _ in 0..5 {
                let response = client.get("http://127.0.0.1:9010/").send().unwrap();
                assert_eq!(response.status(), StatusCode::OK);
                let content_type = response.headers().get(CONTENT_TYPE).unwrap();
                assert_eq!(content_type, "text/plain");
                let body = response.text().unwrap();
                assert_eq!(body, "Hello World\n");
            }
        },
    );
}
