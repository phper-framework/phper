#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// TODO Because `bindgen` generates codes contains deref nullptr, temporary suppression.
#![allow(deref_nullptr)]

/*!
Low level PHP binding for Rust.

The php-config is needed. You can set environment `PHP_CONFIG` to specify the path.

## License

[Unlicense](https://github.com/jmjoy/phper/blob/master/LICENSE).
*/

use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/php_bindings.rs"));

pub const PHP_MODULE_BUILD_ID: *const c_char =
    concat!(env!("PHP_MODULE_BUILD_ID"), "\0").as_ptr().cast();
pub const ZEND_MODULE_BUILD_ID: *const c_char =
    concat!(env!("ZEND_MODULE_BUILD_ID"), "\0").as_ptr().cast();
