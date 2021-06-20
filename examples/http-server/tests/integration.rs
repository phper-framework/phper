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
