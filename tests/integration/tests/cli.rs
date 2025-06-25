// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

mod common;

use crate::common::{DYLIB_PATH, TESTS_PHP_DIR};
use phper_test::cli::test_php_script;

#[test]
fn test_phpinfo() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("phpinfo.php"));
}

#[test]
fn test_arguments() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("arguments.php"));
}

#[test]
fn test_arrays() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("arrays.php"));
}

#[test]
fn test_classes() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("classes.php"));
}

#[test]
fn test_functions() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("functions.php"));
}

#[test]
fn test_objects() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("objects.php"));
}

#[test]
fn test_strings() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("strings.php"));
}

#[test]
fn test_values() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("values.php"));
}

#[test]
fn test_constants() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("constants.php"));
}

#[test]
fn test_ini() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("ini.php"));
}

#[test]
fn test_references() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("references.php"));
}

#[test]
fn test_errors() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("errors.php"));
}

#[test]
fn test_reflection() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("reflection.php"));
}

#[test]
fn test_typehints() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("typehints.php"));
}

#[test]
fn test_enums() {
    test_php_script(&*DYLIB_PATH, TESTS_PHP_DIR.join("enums.php"));
}
