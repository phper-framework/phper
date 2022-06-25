// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper_test::{cli::{test_php_scripts_with_condition, test_php_scripts_with_condition_and_lib}, utils::get_lib_path_by_example};
use std::{env, path::Path, str};

#[test]
fn test_php() {
    let base_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("php");

    test_php_scripts_with_condition_and_lib(
    get_lib_path_by_example(env!("CARGO_BIN_EXE_logging")),
        &[
            (&base_dir.join("test_php_say.php"), &|output| {
                let stdout = str::from_utf8(&output.stdout).unwrap();
                stdout == "Hello, world!" && output.status.success()
            }),
            (&base_dir.join("test_php_notice.php"), &|output| {
                let stdout = str::from_utf8(&output.stdout).unwrap();
                stdout.contains("Notice:")
                    && stdout.contains("Something happened: just for test")
                    && output.status.success()
            }),
            (&base_dir.join("test_php_warning.php"), &|output| {
                let stdout = str::from_utf8(&output.stdout).unwrap();
                stdout.contains("Warning:")
                    && stdout.contains("Something warning: just for test")
                    && output.status.success()
            }),
            (&base_dir.join("test_php_error.php"), &|output| {
                let stdout = str::from_utf8(&output.stdout).unwrap();
                stdout.contains("Fatal error:")
                    && stdout.contains("Something gone failed: just for test")
            }),
            (&base_dir.join("test_php_deprecated.php"), &|output| {
                let stdout = str::from_utf8(&output.stdout).unwrap();
                stdout.contains("Deprecated:")
                    && stdout.contains("Something deprecated: just for test")
                    && output.status.success()
            }),
        ],
    );
}
