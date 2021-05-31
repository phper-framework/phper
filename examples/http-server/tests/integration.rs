use phper_test::test_php_scripts;
use std::{env, path::Path};

#[test]
fn test_php() {
    test_php_scripts(
        env!("CARGO_BIN_EXE_http-server"),
        &[&Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test.php")],
    );
}
