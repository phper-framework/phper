// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use std::{
    convert::TryInto,
    ffi::{OsStr, OsString},
    fmt::Debug,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};

pub fn execute_command<S: AsRef<OsStr> + Debug>(argv: &[S]) -> String {
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]);
    let output = command
        .output()
        .unwrap_or_else(|_| panic!("Execute command {:?} failed", &argv))
        .stdout;
    String::from_utf8(output).unwrap().trim().to_owned()
}

pub fn spawn_command<S: AsRef<OsStr> + Debug>(argv: &[S], wait_time: Option<Duration>) -> Child {
    let mut command = Command::new(&argv[0]);
    let child = command
        .args(&argv[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|_| panic!("Execute command {:?} failed", argv));

    // Sleep to wait program running.
    if let Some(wait_time) = wait_time {
        thread::sleep(wait_time);
    }

    // Check process is running.
    let id = child.id();
    unsafe {
        assert_eq!(
            libc::kill(id.try_into().unwrap(), 0),
            0,
            "start process failed: {:?}",
            argv
        );
    }

    child
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
