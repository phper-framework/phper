// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Test tools for php fpm program.
use crate::{context::Context, utils::spawn_command};
use fastcgi_client::{Client, Params, Request};
use libc::{SIGTERM, atexit, kill, pid_t};
use std::{
    fs,
    path::Path,
    process::Child,
    sync::{Mutex, Once, OnceLock},
    time::Duration,
};
use tempfile::NamedTempFile;
use tokio::{io, net::TcpStream};

static FPM_HANDLE: OnceLock<FpmHandle> = OnceLock::new();

/// A handle for managing a PHP-FPM (FastCGI Process Manager) instance.
///
/// This struct provides functionality to start, manage, and interact with a
/// PHP-FPM process for testing purposes. It maintains the FPM process lifecycle
/// and provides methods to send FastCGI requests to the running FPM instance.
///
/// The FpmHandle is designed as a singleton - only one instance can exist at a
/// time, and it's automatically cleaned up when the program exits.
pub struct FpmHandle {
    /// The running PHP-FPM child process
    fpm_child: Child,
    /// Temporary configuration file for PHP-FPM
    fpm_conf_file: Mutex<Option<NamedTempFile>>,
}

impl FpmHandle {
    /// Sets up and starts a PHP-FPM process for testing.
    ///
    /// This method creates a singleton FpmHandle instance that manages a
    /// PHP-FPM process with the specified PHP extension loaded. The FPM
    /// process is configured to listen on port 9000 and uses a temporary
    /// configuration file.
    ///
    /// # Arguments
    ///
    /// * `lib_path` - Path to the PHP extension library file (.so) to be loaded
    ///
    /// # Returns
    ///
    /// A static reference to the FpmHandle instance
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - PHP-FPM binary cannot be found
    /// - FPM process fails to start
    /// - FpmHandle has already been initialized
    pub fn setup(lib_path: impl AsRef<Path>) -> &'static FpmHandle {
        if FPM_HANDLE.get().is_some() {
            panic!("FPM_HANDLE has set");
        }

        let lib_path = lib_path.as_ref().to_owned();

        // Run php-fpm.
        let context = Context::get_global();
        let php_fpm = context.find_php_fpm().unwrap();
        let fpm_conf_file = context.create_tmp_fpm_conf_file();

        let argv = [
            &*php_fpm,
            "-F",
            "-n",
            "-d",
            &format!("extension={}", lib_path.display()),
            "-y",
            &fpm_conf_file.path().display().to_string(),
        ];
        eprintln!("===== setup php-fpm =====");
        eprintln!("{}", argv.join(" "));

        let child = spawn_command(&argv, Some(Duration::from_secs(3)));
        let log = fs::read_to_string("/tmp/.php-fpm.log").unwrap();
        eprintln!("===== php-fpm log =====");
        eprintln!("{}", log);
        // fs::remove_file("/tmp/.php-fpm.log").unwrap();

        let handle = FpmHandle {
            fpm_child: child,
            fpm_conf_file: Mutex::new(Some(fpm_conf_file)),
        };

        // shutdown hook.
        static TEARDOWN: Once = Once::new();
        TEARDOWN.call_once(|| unsafe {
            atexit(teardown);
        });

        if FPM_HANDLE.set(handle).is_err() {
            panic!("FPM_HANDLE has set");
        }

        FPM_HANDLE.get().unwrap()
    }

    /// Sends a FastCGI request to the PHP-FPM process and validates the
    /// response.
    ///
    /// This method executes a FastCGI request to the running PHP-FPM instance
    /// using the specified parameters. It establishes a TCP connection to
    /// the FPM process and sends the request with the provided HTTP method,
    /// script path, and optional content.
    ///
    /// The method automatically constructs the necessary FastCGI parameters
    /// including script filename, server information, and remote address
    /// details. After receiving the response, it validates that no errors
    /// occurred during processing.
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method for the request (e.g., "GET", "POST", "PUT")
    /// * `root` - Document root directory where PHP scripts are located
    /// * `request_uri` - The URI being requested (e.g.,
    ///   "/test.php?param=value")
    /// * `content_type` - Optional Content-Type header for the request
    /// * `body` - Optional request body as bytes
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - FpmHandle has not been initialized via `setup()` first
    /// - Cannot connect to the FPM process on port 9000
    /// - The PHP script execution results in errors (stderr is not empty)
    pub async fn test_fpm_request(
        &self, method: &str, root: impl AsRef<Path>, request_uri: &str,
        content_type: Option<String>, body: Option<Vec<u8>>,
    ) {
        let root = root.as_ref();
        let script_name = request_uri.split('?').next().unwrap();

        let mut tmp = root.to_path_buf();
        tmp.push(script_name.trim_start_matches('/'));
        let script_filename = tmp.as_path().to_str().unwrap();

        let stream = TcpStream::connect(("127.0.0.1", 9000)).await.unwrap();
        let local_addr = stream.local_addr().unwrap();
        let peer_addr = stream.peer_addr().unwrap();
        let local_ip = local_addr.ip().to_string();
        let local_port = local_addr.port();
        let peer_ip = peer_addr.ip().to_string();
        let peer_port = peer_addr.port();

        let client = Client::new(stream);
        let mut params = Params::default()
            .request_method(method)
            .script_name(request_uri)
            .script_filename(script_filename)
            .request_uri(request_uri)
            .document_uri(script_name)
            .remote_addr(&local_ip)
            .remote_port(local_port)
            .server_addr(&peer_ip)
            .server_port(peer_port)
            .server_name("phper-test");
        if let Some(content_type) = &content_type {
            params = params.content_type(content_type);
        }
        if let Some(body) = &body {
            params = params.content_length(body.len());
        }

        let response = if let Some(body) = body {
            client
                .execute_once(Request::new(params, body.as_ref()))
                .await
        } else {
            client
                .execute_once(Request::new(params, &mut io::empty()))
                .await
        };

        let output = response.unwrap();
        let stdout = output.stdout.unwrap_or_default();
        let stderr = output.stderr.unwrap_or_default();

        let no_error = stderr.is_empty();

        let f = |out: Vec<u8>| {
            String::from_utf8(out)
                .map(|out| {
                    if out.is_empty() {
                        "<empty>".to_owned()
                    } else {
                        out
                    }
                })
                .unwrap_or_else(|_| "<not utf8 string>".to_owned())
        };

        eprintln!("===== request =====");
        eprintln!("{}", request_uri);
        eprintln!("===== stdout ======");
        eprintln!("{}", f(stdout));
        eprintln!("===== stderr ======");
        eprintln!("{}", f(stderr));

        assert!(no_error, "request not success: {}", request_uri);
    }
}

/// Cleanup function called on program exit to properly shutdown the PHP-FPM
/// process.
///
/// This function is automatically registered as an exit handler and is
/// responsible for:
/// - Cleaning up the temporary FPM configuration file
/// - Sending a SIGTERM signal to the FPM process to gracefully shutdown
///
/// # Safety
///
/// This function is marked as `unsafe` because it:
/// - Directly manipulates the global FPM_HANDLE singleton
/// - Uses raw system calls to send signals to processes
/// - Is called from an exit handler context where normal safety guarantees may
///   not apply
extern "C" fn teardown() {
    unsafe {
        let fpm_handle = FPM_HANDLE.get().unwrap();
        drop(fpm_handle.fpm_conf_file.lock().unwrap().take());

        let id = fpm_handle.fpm_child.id();
        kill(id as pid_t, SIGTERM);
    }
}
