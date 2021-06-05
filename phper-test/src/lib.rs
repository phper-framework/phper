#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]

/*!
Integration test tool for [phper](https://crates.io/crates/phper).

The `php-config` is needed. You can set environment `PHP_CONFIG` to specify the path.

## License

[Unlicense](https://github.com/jmjoy/phper/blob/master/LICENSE).
!*/

use crate::context::Context;
use std::{
    path::Path,
    process::{Child, Output},
};

mod context;
mod utils;

/// Check your extension by executing the php script, if the all executing return success, than the test is pass.
///
/// - `exec_path` is the path of the make executable, which will be used to detect the path of
/// extension lib.
///
/// - `scripts` is the path of your php test scripts.
///
/// See [example hello integration test](https://github.com/jmjoy/phper/blob/master/examples/hello/tests/integration.rs).
pub fn test_php_scripts(exe_path: impl AsRef<Path>, scripts: &[&dyn AsRef<Path>]) {
    let condition = |output: Output| output.status.success();
    let scripts = scripts
        .into_iter()
        .map(|s| (*s, &condition as _))
        .collect::<Vec<_>>();
    test_php_scripts_with_condition(exe_path, &*scripts);
}

/// Check your extension by executing the php script, if the all your specified checkers are pass, than the test is pass.
///
/// - `exec_path` is the path of the make executable, which will be used to detect the path of
/// extension lib.
///
/// - `scripts` is the slice of the tuple, format is `(path of your php test script, checker function or closure)`.
///
/// See [example logging integration test](https://github.com/jmjoy/phper/blob/master/examples/logging/tests/integration.rs).
pub fn test_php_scripts_with_condition(
    exe_path: impl AsRef<Path>,
    scripts: &[(&dyn AsRef<Path>, &dyn Fn(Output) -> bool)],
) {
    let context = Context::get_global();
    let lib_path = utils::get_lib_path(exe_path);
    let tmp_php_ini_file = context.create_tmp_php_ini_file(&lib_path);

    for (script, condition) in scripts {
        let mut cmd = context.create_command_with_tmp_php_ini_args(&tmp_php_ini_file, script);

        let output = cmd.output().unwrap();
        let path = script.as_ref().to_str().unwrap();

        let mut stdout = String::from_utf8(output.stdout.clone()).unwrap();
        if stdout.is_empty() {
            stdout.push_str("<empty>");
        }

        let mut stderr = String::from_utf8(output.stderr.clone()).unwrap();
        if stderr.is_empty() {
            stderr.push_str("<empty>");
        }

        println!(
            "===== command =====\n{} {}\n===== stdout ======\n{}\n===== stderr ======\n{}",
            &context.php_bin,
            cmd.get_args().join(" "),
            stdout,
            stderr,
        );
        #[cfg(target_os = "linux")]
        if output.status.code().is_none() {
            use std::os::unix::process::ExitStatusExt;
            println!(
                "===== signal ======\nExitStatusExt is None, the signal is: {:?}",
                output.status.signal()
            );
        }

        if !condition(output) {
            panic!("test php file `{}` failed", path);
        }
    }
}

pub fn test_long_term_php_script_with_condition(
    exe_path: impl AsRef<Path>,
    script: impl AsRef<Path>,
    condition: impl FnOnce(&Child),
) {
    let context = Context::get_global();
    let lib_path = utils::get_lib_path(exe_path);
    let tmp_php_ini_file = context.create_tmp_php_ini_file(lib_path);
    let mut command = context.create_command_with_tmp_php_ini_args(&tmp_php_ini_file, script);
    let mut child = command.spawn().unwrap();
    condition(&child);
    child.kill().unwrap();
}
