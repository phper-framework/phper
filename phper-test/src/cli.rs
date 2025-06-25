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
use log::debug;
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
/// - `script` is the path of your php test script.
pub fn test_php_script(lib_path: impl AsRef<Path>, script: impl AsRef<Path>) {
    let condition = |output: Output| output.status.success();
    let scripts = Some(script);
    let scripts = scripts
        .iter()
        .map(|s| (s as _, &condition as _))
        .collect::<Vec<_>>();
    test_php_scripts_with_condition(lib_path, &scripts);
}

/// Check your extension by executing multiple php scripts with success
/// condition.
///
/// This function executes multiple PHP scripts and checks if all of them return
/// success status. It's a convenience wrapper around
/// `test_php_scripts_with_condition` with a default success condition.
///
/// # Arguments
///
/// * `lib_path` - The path to the extension library file
/// * `scripts` - A slice of references to PHP script paths to execute
///
/// # Panics
///
/// Panics if any script execution fails or returns non-success status.
pub fn test_php_scripts(lib_path: impl AsRef<Path>, scripts: &[&dyn AsRef<Path>]) {
    let condition = |output: Output| output.status.success();
    let scripts = scripts
        .iter()
        .map(|s| (*s, &condition as _))
        .collect::<Vec<_>>();
    test_php_scripts_with_condition(lib_path, &scripts);
}

/// Check your extension by executing a single php script with custom condition.
///
/// This function allows you to specify a custom condition function to check the
/// execution result of a PHP script, providing more flexibility than the
/// default success-only check.
///
/// # Arguments
///
/// * `lib_path` - The path to the extension library file
/// * `script` - The path to the PHP script to execute
/// * `condition` - A function that takes the command output and returns true if
///   the test passes
///
/// # Panics
///
/// Panics if the script execution fails or the condition function returns
/// false.
///
/// # Examples
///
/// ```rust,no_run
/// use phper_test::cli::test_php_script_with_condition;
/// use std::process::Output;
///
/// // Test that script outputs specific text
/// let condition =
///     |output: Output| String::from_utf8_lossy(&output.stdout).contains("expected output");
/// test_php_script_with_condition("/path/to/extension.so", "test.php", condition);
/// ```
pub fn test_php_script_with_condition(
    lib_path: impl AsRef<Path>, script: impl AsRef<Path>, condition: impl Fn(Output) -> bool,
) {
    let scripts = Some(script);
    let scripts = scripts
        .iter()
        .map(|s| (s as _, &condition as _))
        .collect::<Vec<_>>();
    test_php_scripts_with_condition(lib_path, &scripts);
}

/// Type alias for script and condition pair used in batch testing.
///
/// The first element is a reference to a path-like object representing the PHP
/// script, and the second element is a function that validates the execution
/// output.
pub type ScriptCondition<'a> = (&'a dyn AsRef<Path>, &'a dyn Fn(Output) -> bool);

/// Check your extension by executing multiple php scripts with custom
/// conditions.
///
/// This is the most flexible testing function that allows you to specify
/// different validation conditions for each script. It executes each script
/// with the extension loaded and validates the results using the provided
/// condition functions.
///
/// # Arguments
///
/// * `lib_path` - The path to the extension library file
/// * `scripts` - A slice of tuples containing script paths and their validation
///   conditions
///
/// # Panics
///
/// Panics if any script execution fails or if any condition function returns
/// false. The panic message will include the path of the failing script.
///
/// # Examples
///
/// ```rust,no_run
/// use phper_test::cli::{ScriptCondition, test_php_scripts_with_condition};
/// use std::process::Output;
///
/// let success_condition = |output: Output| output.status.success();
/// let custom_condition =
///     |output: Output| String::from_utf8_lossy(&output.stdout).contains("custom check");
///
/// let scripts: &[ScriptCondition] = &[
///     (&"test1.php", &success_condition),
///     (&"test2.php", &custom_condition),
/// ];
/// test_php_scripts_with_condition("/path/to/extension.so", scripts);
/// ```
pub fn test_php_scripts_with_condition(
    lib_path: impl AsRef<Path>, scripts: &[ScriptCondition<'_>],
) {
    let context = Context::get_global();

    for (script, condition) in scripts {
        let mut cmd = context.create_command_with_lib(&lib_path, script);

        let output = cmd.output().unwrap();
        let path = script.as_ref().to_str().unwrap();

        let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
        if stdout.is_empty() {
            stdout.push_str("<empty>");
        }

        let mut stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if stderr.is_empty() {
            stderr.push_str("<empty>");
        };

        debug!(command:% = cmd.get_command().join(" ".as_ref()).to_string_lossy(),
               status:? = output.status.code(),
               stdout = &*stdout,
               stderr:%,
               signal:? = {
                   #[cfg(unix)]
                   {
                       use std::os::unix::process::ExitStatusExt as _;
                       output.status.signal()
                   }
                   #[cfg(not(unix))]
                   {
                       None
                   }
               };
               "execute php test command");

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
