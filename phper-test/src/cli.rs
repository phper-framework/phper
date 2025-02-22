// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Test tools for php cli program.

use crate::context::Context;
use std::{
    panic::{UnwindSafe, catch_unwind, resume_unwind},
    path::Path,
    process::{Child, Output},
};

/// Check your extension by executing the php script, if the all executing
/// return success, than the test is pass.
///
/// - `lib_path` is the path of extension lib.
///
/// - `scripts` is the path of your php test scripts.
///
/// See [example hello integration test](https://github.com/phper-framework/phper/blob/master/examples/hello/tests/integration.rs).
pub fn test_php_scripts(lib_path: impl AsRef<Path>, scripts: &[&dyn AsRef<Path>]) {
    let condition = |output: Output| output.status.success();
    let scripts = scripts
        .iter()
        .map(|s| (*s, &condition as _))
        .collect::<Vec<_>>();
    test_php_scripts_with_condition(lib_path, &scripts);
}

/// Script and condition pair.
pub type ScriptCondition<'a> = (&'a dyn AsRef<Path>, &'a dyn Fn(Output) -> bool);

/// Check your extension by executing the php script, if the all your specified
/// checkers are pass, than the test is pass.
///
/// - `exec_path` is the path of the make executable, which will be used to
///   detect the path of extension lib.
///
/// - `scripts` is the slice of the tuple, format is `(path of your php test
///   script, checker function or closure)`.
///
/// See [example logging integration test](https://github.com/phper-framework/phper/blob/master/examples/logging/tests/integration.rs).
pub fn test_php_scripts_with_condition(
    lib_path: impl AsRef<Path>, scripts: &[ScriptCondition<'_>],
) {
    let context = Context::get_global();

    for (script, condition) in scripts {
        let mut cmd = context.create_command_with_lib(&lib_path, script);

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

        eprintln!(
            "===== command =====\n{} {}\n===== stdout ======\n{}\n===== stderr ======\n{}",
            &context.php_bin,
            cmd.get_args().join(" "),
            stdout,
            stderr,
        );
        #[cfg(target_os = "linux")]
        if output.status.code().is_none() {
            use std::os::unix::process::ExitStatusExt;
            eprintln!(
                "===== signal ======\nExitStatusExt is None, the signal is: {:?}",
                output.status.signal()
            );
        }

        if !condition(output) {
            panic!("test php file `{}` failed", path);
        }
    }
}

/// Check your extension by executing the long term php script such as http
/// server, if the all your specified checkers are pass, than the test is pass.
#[allow(clippy::zombie_processes)]
pub fn test_long_term_php_script_with_condition(
    lib_path: impl AsRef<Path>, script: impl AsRef<Path>,
    condition: impl FnOnce(&Child) + UnwindSafe,
) {
    let context = Context::get_global();
    let mut command = context.create_command_with_lib(lib_path, script);
    let mut child = command.spawn().unwrap();
    let r = catch_unwind(|| condition(&child));
    child.kill().unwrap();
    if let Err(e) = r {
        resume_unwind(e);
    }
}
