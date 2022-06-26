// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper_test::{cli::test_php_scripts_with_lib, utils::get_lib_path_by_example};
use std::{env, path::Path};

#[test]
fn test_php() {
    test_php_scripts_with_lib(
        get_lib_path_by_example(env!("CARGO_BIN_EXE_hello")),
        &[&Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test.php")],
    );
}
