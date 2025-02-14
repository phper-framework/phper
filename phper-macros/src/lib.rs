// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![warn(rust_2018_idioms, missing_docs)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]

// TODO Write a bridge macro for easy usage about register functions and
// classes, like `cxx`.

mod alloc;
mod derives;
mod globals;
mod inner;
mod log;
mod utils;

use proc_macro::TokenStream;

/// PHP module entry, wrap the `phper::modules::Module` write operation.
///
/// # Examples
///
/// ```no_test
/// use phper::{php_get_module, modules::Module};
///
/// #[php_get_module]
/// pub fn get_module() -> Module {
///     let mut module = Module::new(
///         env!("CARGO_CRATE_NAME"),
///         env!("CARGO_PKG_VERSION"),
///         env!("CARGO_PKG_AUTHORS"),
///     );
///
///     // ...
///
///     module
/// }
/// ```
#[proc_macro_attribute]
pub fn php_get_module(attr: TokenStream, input: TokenStream) -> TokenStream {
    inner::php_get_module(attr, input)
}
