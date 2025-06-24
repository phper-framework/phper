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

use crate::common::{FPM_HANDLE, TESTS_PHP_DIR, setup};

#[tokio::test]
async fn test_phpinfo() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/phpinfo.php", None, None)
        .await;
}

#[tokio::test]
async fn test_arguments() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/arguments.php", None, None)
        .await;
}

#[tokio::test]
async fn test_arrays() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/arrays.php", None, None)
        .await;
}

#[tokio::test]
async fn test_classes() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/classes.php", None, None)
        .await;
}

#[tokio::test]
async fn test_functions() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/functions.php", None, None)
        .await;
}

#[tokio::test]
async fn test_objects() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/objects.php", None, None)
        .await;
}

#[tokio::test]
async fn test_strings() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/strings.php", None, None)
        .await;
}

#[tokio::test]
async fn test_values() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/values.php", None, None)
        .await;
}

#[tokio::test]
async fn test_constants() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/constants.php", None, None)
        .await;
}

#[tokio::test]
async fn test_ini() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/ini.php", None, None)
        .await;
}

#[tokio::test]
async fn test_references() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/references.php", None, None)
        .await;
}

#[tokio::test]
async fn test_errors() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/errors.php", None, None)
        .await;
}

#[tokio::test]
async fn test_reflection() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/reflection.php", None, None)
        .await;
}

#[tokio::test]
async fn test_typehints() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/typehints.php", None, None)
        .await;
}

#[tokio::test]
async fn test_enums() {
    setup();
    FPM_HANDLE
        .test_fpm_request("GET", &*TESTS_PHP_DIR, "/enums.php", None, None)
        .await;
}
