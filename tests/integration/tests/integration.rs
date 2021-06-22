use phper_test::{cli::test_php_scripts, fpm, fpm::test_fpm_request};
use std::{env, path::Path};

#[test]
fn test_cli() {
    let tests_php_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("php");

    test_php_scripts(
        env!("CARGO_BIN_EXE_integration"),
        &[
            &tests_php_dir.join("arguments.php"),
            &tests_php_dir.join("arrays.php"),
            &tests_php_dir.join("classes.php"),
            &tests_php_dir.join("functions.php"),
            &tests_php_dir.join("objects.php"),
            &tests_php_dir.join("strings.php"),
            &tests_php_dir.join("values.php"),
        ],
    );
}

#[test]
fn test_fpm() {
    let tests_php_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("php");

    fpm::setup(env!("CARGO_BIN_EXE_integration"));

    test_fpm_request("GET", &tests_php_dir, "/arguments.php", None, None);
    test_fpm_request("GET", &tests_php_dir, "/arrays.php", None, None);
    test_fpm_request("GET", &tests_php_dir, "/classes.php", None, None);
    test_fpm_request("GET", &tests_php_dir, "/functions.php", None, None);
    test_fpm_request("GET", &tests_php_dir, "/objects.php", None, None);
    test_fpm_request("GET", &tests_php_dir, "/strings.php", None, None);
    test_fpm_request("GET", &tests_php_dir, "/values.php", None, None);
}
