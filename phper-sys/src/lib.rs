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
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// TODO Because `bindgen` generates codes contains deref nullptr, temporary suppression.
#![allow(deref_nullptr)]
#![allow(clippy::all)]
#![doc = include_str!("../README.md")]

use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/php_bindings.rs"));

pub const PHP_MODULE_BUILD_ID: *const c_char =
    concat!(env!("PHP_MODULE_BUILD_ID"), "\0").as_ptr().cast();
pub const ZEND_MODULE_BUILD_ID: *const c_char =
    concat!(env!("ZEND_MODULE_BUILD_ID"), "\0").as_ptr().cast();
