use crate::{errors::make_exception_class, server::make_server_class};
use phper::{modules::Module, php_get_module};
use std::{mem::forget, sync::Arc};
use tokio::runtime;

pub mod errors;
pub mod server;
pub mod utils;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let rt = Arc::new(rt);
    let rt_ = rt.clone();

    module.on_module_init(move |_| {
        let guard = rt_.enter();
        forget(guard);
        true
    });

    module.on_module_shutdown(move |_| true);

    module.add_class(make_exception_class());
    module.add_class(make_server_class());

    module
}
