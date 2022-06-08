// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]

mod arguments;
mod arrays;
mod classes;
mod functions;
mod objects;
mod strings;
mod values;

use phper::{modules::Module, php_get_module};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    arguments::integrate(&mut module);
    arrays::integrate(&mut module);
    classes::integrate(&mut module);
    functions::integrate(&mut module);
    objects::integrate(&mut module);
    strings::integrate(&mut module);
    values::integrate(&mut module);

    module
}
