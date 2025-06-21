// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Cargo build utilities for building and analyzing Rust libraries.

use cargo_metadata::Message;
use std::{
    io::{self, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

/// Builder for running cargo build commands with JSON output
pub struct CargoBuilder {
    command: Command,
}

/// Result of a cargo build operation
pub struct CargoBuildResult {
    messages: Vec<Message>,
}

impl CargoBuilder {
    /// Create a new CargoBuilder instance
    pub fn new() -> Self {
        let mut command = Command::new(env!("CARGO"));
        command
            .args(["build", "--lib", "--message-format", "json"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        Self { command }
    }

    /// Add additional arguments to the cargo command
    pub fn arg<S: AsRef<std::ffi::OsStr>>(&mut self, arg: S) -> &mut Self {
        self.command.arg(arg);
        self
    }

    /// Set the current directory for the cargo command
    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.command.current_dir(dir);
        self
    }

    /// Execute the cargo build command and return the result
    pub fn build(&mut self) -> io::Result<CargoBuildResult> {
        let mut child = self.command.spawn()?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to capture stdout"))?;
        let reader = BufReader::new(stdout);
        let mut messages = Vec::new();
        for message in cargo_metadata::Message::parse_stream(reader) {
            let message = message?;
            messages.push(message);
        }
        let exit_status = child.wait()?;
        if !exit_status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Cargo build failed with exit status: {}", exit_status),
            ));
        }
        Ok(CargoBuildResult { messages })
    }
}

impl Default for CargoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CargoBuildResult {
    /// Get the cdylib file path from the last compiler-artifact message
    pub fn get_cdylib(&self) -> Option<PathBuf> {
        self.messages.iter().rev().find_map(|msg| {
            if let Message::CompilerArtifact(artifact) = msg {
                artifact.filenames.iter().find_map(|filename| {
                    let ext = filename.extension();
                    if matches!(ext, Some("so") | Some("dylib") | Some("dll")) {
                        Some(PathBuf::from(filename.as_std_path()))
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
    }
}
