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
use phper_test::{cli::test_long_term_php_script_with_condition, utils::get_lib_path};
use reqwest::blocking::Client;
use std::{
    env,
    path::{Path, PathBuf},
    thread::sleep,
    time::Duration,
};

#[test]
fn test_php() {
    test_long_term_php_script_with_condition(
        get_lib_path(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("target"),
            "http_server",
        ),
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test.php"),
        |_| {
            // wait for server startup.
            sleep(Duration::from_secs(3));

            let client = Client::new();
            for _ in 0..5 {
                let response = client.get("http://127.0.0.1:9000/").send().unwrap();
                let content_type = response.headers().get(CONTENT_TYPE).unwrap();
                assert_eq!(content_type, "text/plain");
                let body = response.text().unwrap();
                assert_eq!(body, "Hello World\n");
            }
        },
    );
}
