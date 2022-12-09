// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use bindgen::Builder;
use std::{env, ffi::OsStr, fmt::Debug, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=php_wrapper.c");
    println!("cargo:rerun-if-env-changed=PHP_CONFIG");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let php_config = env::var("PHP_CONFIG").unwrap_or_else(|_| "php-config".to_string());

    let includes = execute_command(&[php_config.as_str(), "--includes"]);
    let includes = includes.split(' ').collect::<Vec<_>>();

    // Generate libphpwrapper.a.

    let mut builder = cc::Build::new();
    for include in &includes {
        builder.flag(include);
    }
    builder.file("php_wrapper.c").compile("phpwrapper");

    // Generate bindgen file.
    let include_dirs = includes
        .iter()
        .map(|include| &include[2..])
        .collect::<Vec<_>>();

    for dir in include_dirs.iter() {
        println!("cargo:include={}", dir);
    }

    let mut builder = Builder::default()
        .header("php_wrapper.c")
        .allowlist_file("php_wrapper\\.c")
        // Block the `zend_ini_parse_quantity` because it's document causes the doc test to fail.
        .blocklist_function("zend_ini_parse_quantity")
        .clang_args(&includes)
        .derive_default(true);

    // iterate over the php include directories, and update the builder
    // to only create bindings from the header files in those directories
    for dir in include_dirs.iter() {
        let p = PathBuf::from(dir).join(".*\\.h");
        builder = builder.allowlist_file(p.to_str().unwrap());
    }

    let generated_path = out_path.join("php_bindings.rs");

    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&generated_path)
        .expect("Unable to write output file");
}

fn execute_command<S: AsRef<OsStr> + Debug>(argv: &[S]) -> String {
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]);
    let output = command
        .output()
        .unwrap_or_else(|_| panic!("Execute command {:?} failed", &argv))
        .stdout;
    String::from_utf8(output).unwrap().trim().to_owned()
}
