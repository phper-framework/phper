use std::{
    ffi::{OsStr, OsString},
    fmt::Debug,
    path::{Path, PathBuf},
    process::Command,
};

pub fn execute_command<S: AsRef<OsStr> + Debug>(argv: &[S]) -> String {
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]);
    let output = command
        .output()
        .expect(&format!("Execute command {:?} failed", &argv))
        .stdout;
    String::from_utf8(output).unwrap().trim().to_owned()
}

pub fn get_lib_path(exe_path: impl AsRef<Path>) -> PathBuf {
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
