// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use bindgen::{
    Builder,
    callbacks::{DeriveTrait, ImplementsTrait, ParseCallbacks},
};
use std::{env, ffi::OsStr, fmt::Debug, fs, path::PathBuf, process::Command};

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
        // Block the `zend_startup` because it fails checks.
        .blocklist_function("zend_startup")
        // Block the `zend_random_bytes_insecure` because it fails checks.
        .blocklist_item("zend_random_bytes_insecure")
        .clang_args(&includes)
        .derive_default(true);

    // iterate over the php include directories, and update the builder
    // to only create bindings from the header files in those directories
    for dir in include_dirs.iter() {
        let p = PathBuf::from(dir).join(".*\\.h");
        builder = builder.allowlist_file(p.display().to_string());
    }

    // From PHP 8.5, a build where configure finds `preserve_none` may declare
    // opcode handlers with it. It has no Rust ABI, and bindgen panics on it.
    // Nothing in Rust can call a handler, so make it the `const void *` that
    // HYBRID builds already get.
    let include_dir = execute_command(&[php_config.as_str(), "--include-dir"]);
    let php_config_h =
        fs::read_to_string(PathBuf::from(include_dir).join("main/php_config.h")).unwrap();
    if php_config_h.lines().any(|line| {
        line.split_whitespace()
            .take(2)
            .eq(["#define", "HAVE_PRESERVE_NONE"])
    }) {
        builder = builder
            .blocklist_type("zend_vm_opcode_handler_t")
            .raw_line("pub type zend_vm_opcode_handler_t = *const ::std::os::raw::c_void;")
            .parse_callbacks(Box::new(OpaqueOpcodeHandler));
    }

    let generated_path = out_path.join("php_bindings.rs");

    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(generated_path)
        .expect("Unable to write output file");
}

/// Without this, `_zend_op` and every struct holding one lose their derives,
/// since bindgen assumes a blocklisted type implements nothing.
#[derive(Debug)]
struct OpaqueOpcodeHandler;

impl ParseCallbacks for OpaqueOpcodeHandler {
    fn blocklisted_type_implements_trait(
        &self, name: &str, derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        if name != "zend_vm_opcode_handler_t" {
            return None;
        }
        Some(match derive_trait {
            DeriveTrait::Default => ImplementsTrait::Manually,
            _ => ImplementsTrait::Yes,
        })
    }
}

fn execute_command<S: AsRef<OsStr> + Debug>(argv: &[S]) -> String {
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]);
    let output = command
        .output()
        .unwrap_or_else(|e| {
            panic!(
                "Failed to execute `{:?}`: {}. Is php-config installed and in PATH?",
                &argv, e
            )
        })
        .stdout;
    String::from_utf8(output).unwrap().trim().to_owned()
}
