// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::utils;
use ini::Ini;
use std::{
    env,
    ffi::OsStr,
    fs::read_to_string,
    ops::{Deref, DerefMut},
    path::Path,
    process::Command,
    sync::OnceLock,
};
use tempfile::NamedTempFile;

pub struct Context {
    pub php_bin: String,
    #[allow(dead_code)]
    pub ini_content: String,
}

impl Context {
    pub fn get_global() -> &'static Context {
        static CONTEXT: OnceLock<Context> = OnceLock::new();
        CONTEXT.get_or_init(|| {
            let mut ini_content = String::new();

            let php_config = env::var("PHP_CONFIG").unwrap_or_else(|_| "php-config".to_string());
            let php_bin = utils::execute_command(&[php_config.as_str(), "--php-binary"]);
            let ini_file = utils::execute_command(&[
                php_bin.as_str(),
                "-d",
                "display_errors=stderr",
                "-r",
                "echo php_ini_loaded_file();",
            ]);
            let ini_files = utils::execute_command(&[
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

    pub fn create_command_with_lib(
        &self, lib_path: impl AsRef<Path>, script: impl AsRef<Path>,
    ) -> ContextCommand {
        let mut cmd = Command::new(&self.php_bin);
        let args = vec![
            "-n".to_owned(),
            "-d".to_owned(),
            format!("extension={}", lib_path.as_ref().display()),
            script.as_ref().display().to_string(),
        ];
        cmd.args(&args);
        ContextCommand { cmd }
    }

    pub fn find_php_fpm(&self) -> Option<String> {
        use std::ffi::OsStr;

        if let Ok(php_fpm_bin) = env::var("PHP_FPM_BIN") {
            return Some(php_fpm_bin);
        }

        let php_bin = Path::new(&self.php_bin);
        php_bin.parent().and_then(Path::parent).and_then(|p| {
            php_bin
                .file_name()
                .and_then(OsStr::to_str)
                .and_then(|name| {
                    let mut p = p.to_path_buf();
                    p.push("sbin");
                    p.push(format!(
                        "php-fpm{}",
                        if name.starts_with("php") {
                            name.chars().skip(3).collect::<String>()
                        } else {
                            "".to_owned()
                        }
                    ));
                    p.as_path().to_str().map(ToOwned::to_owned)
                })
        })
    }

    pub fn create_tmp_fpm_conf_file(&self, port: u16, error_log: &Path) -> NamedTempFile {
        let mut tmp = NamedTempFile::new().unwrap();
        let file = tmp.as_file_mut();

        let mut conf = Ini::new();
        conf.with_section(Some("global"))
            .set("error_log", error_log.display().to_string());
        conf.with_section(Some("www"))
            .set("user", "$USER")
            .set("group", "$USER")
            .set("listen", format!("127.0.0.1:{port}"))
            .set("pm", "static")
            .set("pm.max_children", "6")
            .set("pm.max_requests", "500");

        conf.write_to(file).unwrap();

        tmp
    }
}

pub struct ContextCommand {
    cmd: Command,
}

impl ContextCommand {
    pub fn get_command(&self) -> Vec<&OsStr> {
        let program = self.cmd.get_program();
        let args = self.cmd.get_args();
        let mut command = vec![program];
        command.extend(args);
        command
    }
}

impl Deref for ContextCommand {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.cmd
    }
}

impl DerefMut for ContextCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cmd
    }
}
