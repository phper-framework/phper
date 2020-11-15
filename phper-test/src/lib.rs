use once_cell::sync::OnceCell;
use serde_json::Value;
use std::{
    env, ffi::OsStr, fmt::Debug, fs::read_to_string, io, io::Write, path::Path, process::Command,
};
use tempfile::{tempfile, NamedTempFile};

pub fn test_php_scripts(
    target_dir: impl AsRef<Path>,
    lib_name: &str,
    scripts: &[impl AsRef<Path>],
) {
    let context = php_context();

    let mut lib_path = target_dir
        .as_ref()
        .join(if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        })
        .join(format!("lib{}.so", lib_name));

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
        cmd.arg("-n")
            .arg("-c")
            .arg(out_ini_temp_file.path())
            .arg(script);
        let output = cmd.output().unwrap();
        if !output.status.success() {
            eprintln!(
                "stdout: {}\nstderr: {}",
                String::from_utf8(output.stdout).unwrap(),
                String::from_utf8(output.stderr).unwrap()
            );
            panic!("test php file `{}` failed", script.to_str().unwrap());
        }
    }
}

struct Context {
    php_config: String,
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
            ini_content.push_str(&read_to_string(file).unwrap());
        }

        Context {
            php_config,
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
