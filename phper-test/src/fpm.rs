//! Test tools for php fpm program.
use crate::{context::Context, utils, utils::spawn_command};
use fastcgi_client::{Client, Params, Request};
use libc::{atexit, kill, pid_t, SIGTERM};
use once_cell::sync::OnceCell;
use std::{
    fs,
    mem::{forget, ManuallyDrop},
    path::{Path, PathBuf},
    process::Child,
    sync::Mutex,
    time::Duration,
};
use tempfile::NamedTempFile;
use tokio::{io, net::TcpStream, runtime::Handle, task::block_in_place};

static FPM_HANDLE: OnceCell<Mutex<FpmHandle>> = OnceCell::new();

struct FpmHandle {
    exe_path: PathBuf,
    fpm_child: Child,
    php_ini_file: ManuallyDrop<NamedTempFile>,
    fpm_conf_file: ManuallyDrop<NamedTempFile>,
}

/// Start php-fpm process and tokio runtime.
pub fn setup(exe_path: impl AsRef<Path>) {
    let exe_path = exe_path.as_ref();

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
        let lib_path = utils::get_lib_path(exe_path);
        let php_fpm = context.find_php_fpm().unwrap();
        let php_ini_file = context.create_tmp_php_ini_file(&lib_path);
        let fpm_conf_file = context.create_tmp_fpm_conf_file();

        let argv = [
            &*php_fpm,
            "-F",
            "-n",
            "-c",
            php_ini_file.path().to_str().unwrap(),
            "-y",
            fpm_conf_file.path().to_str().unwrap(),
        ];
        println!("===== setup php-fpm =====\n{}", argv.join(" "));

        let child = spawn_command(&argv, Some(Duration::from_secs(3)));
        let log = fs::read_to_string("/tmp/.php-fpm.log").unwrap();
        println!("===== php-fpm log =====\n{}", log);
        // fs::remove_file("/tmp/.php-fpm.log").unwrap();

        Mutex::new(FpmHandle {
            exe_path: exe_path.into(),
            fpm_child: child,
            php_ini_file: ManuallyDrop::new(php_ini_file),
            fpm_conf_file: ManuallyDrop::new(fpm_conf_file),
        })
    });

    assert_eq!(&handle.lock().unwrap().exe_path, exe_path);
}

extern "C" fn teardown() {
    let mut fpm_handle = FPM_HANDLE.get().unwrap().lock().unwrap();

    unsafe {
        ManuallyDrop::drop(&mut fpm_handle.php_ini_file);
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
            let stdout = output.get_stdout().unwrap_or_default();
            let stderr = output.get_stderr().unwrap_or_default();

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

            println!(
                "===== request =====\n{}\n===== stdout ======\n{}\n===== stderr ======\n{}",
                request_uri,
                f(stdout),
                f(stderr),
            );

            assert!(no_error, "request not success: {}", request_uri);
        });
    });
}
