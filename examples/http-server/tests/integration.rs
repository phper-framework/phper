// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use hyper::header::CONTENT_TYPE;
use phper_test::cli::test_long_term_php_script_with_condition;
use reqwest::Client;
use std::{env, path::Path, thread::sleep, time::Duration};
use tokio::runtime;

#[test]
fn test_php() {
    test_long_term_php_script_with_condition(
        env!("CARGO_BIN_EXE_http-server"),
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test.php"),
        |_| {
            // wait for server startup.
            sleep(Duration::from_secs(3));

            runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let client = Client::new();
                    let response = client.get("http://127.0.0.1:9000/").send().await.unwrap();
                    let content_type = response.headers().get(CONTENT_TYPE).unwrap();
                    assert_eq!(content_type, "text/plain");
                    let body = response.text().await.unwrap();
                    assert_eq!(body, "Hello World\n");
                });
        },
    );
}
