use phper_test::test_php_scripts;
use std::{env, path::Path};

#[test]
fn test_php() {
    test_php_scripts(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("target"),
        env!("CARGO_PKG_NAME"),
        &[Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test.php")],
    );
}
