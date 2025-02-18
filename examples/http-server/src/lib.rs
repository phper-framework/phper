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

pub mod errors;
pub mod request;
pub mod response;
pub mod server;

#[php_get_module]
pub fn get_module() -> Module {
    // Add module info.
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Register classes.
    module.add_class(make_exception_class());
    let request_class = module.add_class(make_request_class());
    let response_class = module.add_class(make_response_class());
    module.add_class(make_server_class(request_class, response_class));

    module
}
