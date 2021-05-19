#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]

/*!
Integration test tool for [phper](https://crates.io/crates/phper).

The `php-config` is needed. You can set environment `PHP_CONFIG` to specify the path.

## License

[Unlicense](https://github.com/jmjoy/phper/blob/master/LICENSE).
!*/

use once_cell::sync::OnceCell;
use std::{
    env,
    ffi::{OsStr, OsString},
    fmt::Debug,
    fs::read_to_string,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use tempfile::NamedTempFile;

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
    let context = php_context();

    let lib_path = get_lib_path(exe_path);

    let mut out_ini_temp_file = NamedTempFile::new().unwrap();
    let out_ini_file = out_ini_temp_file.as_file_mut();

    out_ini_file
        .write_all(context.ini_content.as_bytes())
        .unwrap();
    out_ini_file
        .write_fmt(format_args!("extension={}\n", lib_path.to_str().unwrap()))
        .unwrap();

    for (script, condition) in scripts {
        let script = script.as_ref();
        let mut cmd = Command::new(&context.php_bin);
        let args = &[
            "-n",
            "-c",
            out_ini_temp_file.path().to_str().unwrap(),
            script.to_str().unwrap(),
        ];
        cmd.args(args);
        let output = cmd.output().unwrap();
        let path = script.to_str().unwrap();

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
            args.join(" "),
            stdout,
            stderr,
        );
        if !condition(output) {
            panic!("test php file `{}` failed", path);
        }
    }
}

struct Context {
    php_bin: String,
    ini_content: String,
}

fn php_context() -> &'static Context {
    static CONTEXT: OnceCell<Context> = OnceCell::new();
    &CONTEXT.get_or_init(|| {
        let mut ini_content = String::new();

        let php_config = env::var("PHP_CONFIG").unwrap_or("php-config".to_string());
        let php_bin = execute_command(&[php_config.as_str(), "--php-binary"]);
        let ini_file = execute_command(&[
            php_bin.as_str(),
            "-d",
            "display_errors=stderr",
            "-r",
            "echo php_ini_loaded_file();",
        ]);
        let ini_files = execute_command(&[
            php_bin.as_str(),
            "-d",
            "display_errors=stderr",
            "-r",
            "echo php_ini_scanned_files();",
        ]);

        if !ini_file.is_empty() {
            ini_content.push_str(&read_to_string(ini_file).unwrap());
        }
        if !ini_files.is_empty() {
            for file in ini_files.split(',') {
                let file = file.trim();
                if !file.is_empty() {
                    ini_content.push_str(&read_to_string(file).unwrap());
                }
            }
        }

        Context {
            php_bin,
            ini_content,
        }
    })
}

fn execute_command<S: AsRef<OsStr> + Debug>(argv: &[S]) -> String {
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]);
    let output = command
        .output()
        .expect(&format!("Execute command {:?} failed", &argv))
        .stdout;
    String::from_utf8(output).unwrap().trim().to_owned()
}

fn get_lib_path(exe_path: impl AsRef<Path>) -> PathBuf {
    let exe_path = exe_path.as_ref();
    let exe_stem = exe_path
        .file_stem()
        .expect("failed to get current exe stem")
        .to_str()
        .expect("failed to convert to utf-8 str");
    let target_dir = exe_path
        .parent()
        .expect("failed to get current exe directory");

    let mut ext_name = OsString::new();
    ext_name.push("lib");
    ext_name.push(exe_stem.replace('-', "_"));
    #[cfg(target_os = "linux")]
    ext_name.push(".so");
    #[cfg(target_os = "macos")]
    ext_name.push(".dylib");
    #[cfg(target_os = "windows")]
    ext_name.push(".dll");

    target_dir.join(ext_name)
}
