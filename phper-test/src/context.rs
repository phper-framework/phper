use crate::utils;
use once_cell::sync::OnceCell;
use std::{
    env,
    fs::read_to_string,
    io::Write,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    process::Command,
};
use tempfile::NamedTempFile;

pub struct Context {
    pub php_bin: String,
    pub ini_content: String,
}

impl Context {
    pub fn get_global() -> &'static Context {
        static CONTEXT: OnceCell<Context> = OnceCell::new();
        &CONTEXT.get_or_init(|| {
            let mut ini_content = String::new();

            let php_config = env::var("PHP_CONFIG").unwrap_or("php-config".to_string());
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

    pub fn create_tmp_php_ini_file(&self, lib_path: impl AsRef<Path>) -> NamedTempFile {
        let mut out_ini_temp_file = NamedTempFile::new().unwrap();
        let out_ini_file = out_ini_temp_file.as_file_mut();

        out_ini_file.write_all(self.ini_content.as_bytes()).unwrap();
        out_ini_file
            .write_fmt(format_args!(
                "extension={}\n",
                lib_path.as_ref().to_str().unwrap()
            ))
            .unwrap();

        out_ini_temp_file
    }

    pub fn create_command_with_tmp_php_ini_args(
        &self,
        tmp_php_ini_file: &NamedTempFile,
        script: impl AsRef<Path>,
    ) -> ContextCommand {
        let mut cmd = Command::new(&self.php_bin);
        let args = vec![
            "-n".to_owned(),
            "-c".to_owned(),
            tmp_php_ini_file.path().to_str().unwrap().to_owned(),
            script.as_ref().to_str().unwrap().to_owned(),
        ];
        cmd.args(&args);
        ContextCommand { cmd, args }
    }

    #[cfg(feature = "fpm")]
    #[cfg_attr(docsrs, doc(cfg(feature = "fpm")))]
    pub fn find_php_fpm(&self) -> Option<String> {
        Path::new(&self.php_bin)
            .parent()
            .and_then(Path::parent)
            .and_then(|p| {
                let mut p = p.to_path_buf();
                p.push("sbin");
                p.push("php-fpm");
                p.as_path().to_str().map(|s| s.to_string())
            })
    }
}

pub struct ContextCommand {
    cmd: Command,
    args: Vec<String>,
}

impl ContextCommand {
    pub fn get_args(&self) -> &[String] {
        &self.args
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
