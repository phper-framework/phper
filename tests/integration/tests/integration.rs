use phper_test::test_php_scripts;
use std::{env, path::Path};

#[test]
fn test_php() {
    let tests_php_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("php");

    test_php_scripts(
        env!("CARGO_BIN_EXE_integration"),
        &[
            &tests_php_dir.join("arguments.php"),
            &tests_php_dir.join("arrays.php"),
            &tests_php_dir.join("classes.php"),
            &tests_php_dir.join("objects.php"),
            &tests_php_dir.join("strings.php"),
            &tests_php_dir.join("values.php"),
        ],
    );
}
