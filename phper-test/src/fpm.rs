//! Test tools for php fpm program.
//!
use crate::context::Context;
use fastcgi_client::{Client, Params, Request};
use libc::{atexit, kill, pid_t, SIGTERM};
use once_cell::sync::OnceCell;
use std::{
    cell::RefCell,
    mem::forget,
    path::Path,
    process::{Child, Command, Stdio},
    sync::Mutex,
    thread,
    thread::sleep,
    time::Duration,
};
use tokio::{io, io::AsyncRead, net::TcpStream, runtime::Handle, task::block_in_place};

static FPM_COMMAND: OnceCell<Child> = OnceCell::new();

/// Start php-fpm process and tokio runtime.
fn setup() {
    FPM_COMMAND.get_or_init(|| {
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
        let mut cmd = Command::new(php_fpm);
        // TODO add php-fpm config and php.ini.
        let child = cmd
            .args(&["-F"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("start php-fpm");

        // Sleep 3 seconds to wait php-fpm running.
        thread::sleep(Duration::from_secs(3));

        child
    });
}

extern "C" fn teardown() {
    let id = FPM_COMMAND.get().unwrap().id();
    unsafe {
        kill(id as pid_t, SIGTERM);
    }
}

/// Start php-fpm and test the url request.
pub fn test_fpm_request(
    method: &str,
    root: impl AsRef<Path>,
    request_uri: &str,
    content_type: Option<String>,
    body: Option<Vec<u8>>,
) {
    setup();

    block_in_place(move || {
        Handle::current().block_on(async move {
            let root = root.as_ref();
            let script_name = request_uri.split('?').nth(0).unwrap();

            let mut tmp = root.to_path_buf();
            tmp.push(script_name.trim_start_matches('/'));
            let script_filename = tmp.as_path().to_str().unwrap();

            let stream = TcpStream::connect(("127.0.0.1", 9000)).await.unwrap();
            let local_addr = stream.local_addr().unwrap();
            let peer_addr = stream.peer_addr().unwrap();
            let local_ip = local_addr.ip().to_string();
            let local_addr = local_addr.port().to_string();
            let peer_ip = peer_addr.ip().to_string();
            let peer_port = peer_addr.port().to_string();

            let mut client = Client::new(stream, false);
            let mut params = Params::default()
                .set_request_method(method)
                .set_script_name(request_uri)
                .set_script_filename(script_filename)
                .set_request_uri(request_uri)
                .set_document_uri(script_name)
                .set_remote_addr(&local_ip)
                .set_remote_port(&local_addr)
                .set_server_addr(&peer_ip)
                .set_server_port(&peer_port)
                .set_server_name("phper-test");
            if let Some(content_type) = &content_type {
                params = params.set_content_type(content_type);
            }
            let mut len = String::new();
            if let Some(body) = &body {
                len += &body.len().to_string();
                params = params.set_content_length(&len);
            }

            let response = if let Some(body) = body {
                client.execute(Request::new(params, body.as_ref())).await
            } else {
                client.execute(Request::new(params, &mut io::empty())).await
            };

            let output = response.unwrap();
            let stdout = String::from_utf8(output.get_stdout().unwrap()).unwrap();
            dbg!(stdout);
        });
    });
}
