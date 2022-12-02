// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use std::{
    ffi::{OsStr, OsString},
    fmt::Debug,
    path::{Path, PathBuf},
    process::Command,
};

pub(crate) fn execute_command<S: AsRef<OsStr> + Debug>(argv: &[S]) -> String {
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]);
    let output = command
        .output()
        .unwrap_or_else(|_| panic!("Execute command {:?} failed", &argv))
        .stdout;
    String::from_utf8(output).unwrap().trim().to_owned()
}

#[cfg(feature = "fpm")]
pub(crate) fn spawn_command<S: AsRef<OsStr> + Debug>(
    argv: &[S], wait_time: Option<std::time::Duration>,
) -> std::process::Child {
    use std::{process::Stdio, thread};

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

pub fn get_lib_path(target_path: impl AsRef<Path>, package_name: &str) -> PathBuf {
    let target_path = target_path.as_ref();
    let mut path = target_path.to_path_buf();
    path.push(if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    });
    let mut ext_name = OsString::new();
    ext_name.push("lib");
    ext_name.push(package_name);
    ext_name.push(if cfg!(target_os = "linux") {
        ".so"
    } else if cfg!(target_os = "macos") {
        ".dylib"
    } else if cfg!(target_os = "windows") {
        ".dll"
    } else {
        ""
    });
    path.join(ext_name)
}
