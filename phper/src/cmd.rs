//! Command tools for build, test and install extension process.

use crate::sys::PHP_EXTENSION_DIR;
use anyhow::Context;
use clap::Parser;
use std::{
    env,
    ffi::{CStr, OsString},
    fs,
    path::{Path, PathBuf},
};

/// Make utility.
#[derive(Parser)]
struct Make {
    #[clap(subcommand)]
    sub: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Install(InstallCommand),
}

#[derive(Parser)]
struct InstallCommand {}

/// Make.
///
/// # Examples
///
/// ```no_run
/// use phper::cmd::make;
///
/// fn main() {
///     make();
/// }
/// ```
#[allow(clippy::needless_doctest_main)]
pub fn make() {
    try_make().expect("make failed");
}

pub fn try_make() -> crate::Result<()> {
    let make: Make = Make::parse();
    match make.sub {
        SubCommand::Install(_) => {
            let (lib_path, ext_name) = get_lib_path_and_ext_name()?;
            let extension_dir = CStr::from_bytes_with_nul(PHP_EXTENSION_DIR)?.to_str()?;
            println!("Installing shared extensions:     {}", extension_dir);
            let ext_path = Path::new(extension_dir).join(ext_name);
            fs::create_dir_all(extension_dir)?;
            fs::copy(lib_path, ext_path)?;
        }
    }
    Ok(())
}

fn get_lib_path_and_ext_name() -> crate::Result<(PathBuf, OsString)> {
    let exe_path = env::current_exe()?;
    let exe_stem = exe_path
        .file_stem()
        .context("failed to get current exe stem")?;
    let target_dir = exe_path
        .parent()
        .context("failed to get current exe directory")?;

    #[cfg(target_os = "linux")]
    let ext = ".so";
    #[cfg(target_os = "macos")]
    let ext = ".dylib";
    #[cfg(target_os = "windows")]
    let ext = ".dll";

    let mut exe_name = OsString::new();
    exe_name.push("lib");
    let lib_stem = exe_stem
        .to_str()
        .context("failed to generate target lib name")?
        .replace('-', "_");
    exe_name.push(lib_stem);
    exe_name.push(ext);

    let mut ext_name = OsString::new();
    ext_name.push(exe_stem);
    ext_name.push(ext);

    Ok((target_dir.join(exe_name), ext_name))
}
