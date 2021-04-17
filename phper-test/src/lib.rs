use once_cell::sync::OnceCell;
use std::{
    env, ffi::OsStr, fmt::Debug, fs::read_to_string, io::Write, path::Path, process::Command,
};
use tempfile::NamedTempFile;
use std::ffi::OsString;
use std::path::PathBuf;
use std::error::Error;

pub fn test_php_scripts(
    exe_path: impl AsRef<Path>,
    scripts: &[impl AsRef<Path>],
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

    for script in scripts {
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

        let mut stdout = String::from_utf8(output.stdout).unwrap();
        if stdout.is_empty() {
            stdout.push_str("<empty>");
        }

        let mut stderr = String::from_utf8(output.stderr).unwrap();
        if stderr.is_empty() {
            stderr.push_str("<empty>");
        }

        println!(
            "command: {} {}\nstdout: {}\nstderr: {}",
            &context.php_bin,
            args.join(" "),
            stdout,
            stderr,
        );
        if !output.status.success() {
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

        ini_content.push_str(&read_to_string(ini_file).unwrap());
        for file in ini_files.split(',') {
            let file = file.trim();
            if !file.is_empty() {
                ini_content.push_str(&read_to_string(file).unwrap());
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
    let exe_stem = exe_path.file_stem().expect("failed to get current exe stem");
    let target_dir = exe_path.parent().expect("failed to get current exe directory");

    let mut ext_name = OsString::new();
    ext_name.push("lib");
    ext_name.push(exe_stem);
    ext_name.push(".so");

    target_dir.join(ext_name)
}
