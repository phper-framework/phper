// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::{
    errors::make_exception_class, request::make_request_class, response::make_response_class,
    server::make_server_class,
};
use phper::{modules::Module, php_get_module};
use std::{mem::forget, sync::Arc};
use tokio::runtime;

pub mod errors;
pub mod request;
pub mod response;
pub mod server;
pub mod utils;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let rt = Arc::new(rt);
    let rt_ = rt.clone();

    module.on_module_init(move || {
        let guard = rt_.enter();
        forget(guard);
        true
    });
    module.on_module_shutdown(move || {
        drop(rt);
        true
    });

    module.add_class(make_exception_class());
    module.add_class(make_server_class());
    module.add_class(make_request_class());
    module.add_class(make_response_class());

    module
}
