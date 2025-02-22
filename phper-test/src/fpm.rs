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
use once_cell::sync::OnceCell;
use std::{
    fs,
    mem::{ManuallyDrop, forget},
    path::{Path, PathBuf},
    process::Child,
    sync::Mutex,
    time::Duration,
};
use tempfile::NamedTempFile;
use tokio::{io, net::TcpStream, runtime::Handle, task::block_in_place};

static FPM_HANDLE: OnceCell<Mutex<FpmHandle>> = OnceCell::new();

struct FpmHandle {
    lib_path: PathBuf,
    fpm_child: Child,
    fpm_conf_file: ManuallyDrop<NamedTempFile>,
}

/// Start php-fpm process and tokio runtime.
pub fn setup(lib_path: impl AsRef<Path>) {
    let lib_path = lib_path.as_ref().to_owned();

    let handle = FPM_HANDLE.get_or_init(|| {
        // shutdown hook.
        unsafe {
            atexit(teardown);
        }

        // Run tokio runtime.
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(3)
            .enable_all()
            .build()
            .unwrap();
        let guard = rt.enter();
        forget(guard);
        forget(rt);

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
            fpm_conf_file.path().to_str().unwrap(),
        ];
        eprintln!("===== setup php-fpm =====\n{}", argv.join(" "));

        let child = spawn_command(&argv, Some(Duration::from_secs(3)));
        let log = fs::read_to_string("/tmp/.php-fpm.log").unwrap();
        eprintln!("===== php-fpm log =====\n{}", log);
        // fs::remove_file("/tmp/.php-fpm.log").unwrap();

        Mutex::new(FpmHandle {
            lib_path: lib_path.clone(),
            fpm_child: child,
            fpm_conf_file: ManuallyDrop::new(fpm_conf_file),
        })
    });

    assert_eq!(handle.lock().unwrap().lib_path, &*lib_path);
}

extern "C" fn teardown() {
    let mut fpm_handle = FPM_HANDLE.get().unwrap().lock().unwrap();

    unsafe {
        ManuallyDrop::drop(&mut fpm_handle.fpm_conf_file);

        let id = fpm_handle.fpm_child.id();
        kill(id as pid_t, SIGTERM);
    }
}

/// Start php-fpm and test the url request.
pub fn test_fpm_request(
    method: &str, root: impl AsRef<Path>, request_uri: &str, content_type: Option<String>,
    body: Option<Vec<u8>>,
) {
    assert!(FPM_HANDLE.get().is_some(), "must call `setup()` first");

    block_in_place(move || {
        Handle::current().block_on(async move {
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

            eprintln!(
                "===== request =====\n{}\n===== stdout ======\n{}\n===== stderr ======\n{}",
                request_uri,
                f(stdout),
                f(stderr),
            );

            assert!(no_error, "request not success: {}", request_uri);
        });
    });
}
