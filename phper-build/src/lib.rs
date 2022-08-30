// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![warn(rust_2018_idioms, clippy::dbg_macro)]
#![doc = include_str!("../README.md")]

use phper_sys::*;

/// Register useful rust cfg for project using phper.
pub fn register_configures() {
    // versions
    println!(
        "cargo:rustc-cfg=phper_major_version=\"{}\"",
        PHP_MAJOR_VERSION
    );
    println!(
        "cargo:rustc-cfg=phper_minor_version=\"{}\"",
        PHP_MINOR_VERSION
    );
    println!(
        "cargo:rustc-cfg=phper_release_version=\"{}\"",
        PHP_RELEASE_VERSION
    );
    println!(
        "cargo:rustc-cfg=phper_php_version=\"{}.{}\"",
        PHP_MAJOR_VERSION, PHP_MINOR_VERSION,
    );

    if PHP_DEBUG != 0 {
        println!("cargo:rustc-cfg=phper_debug");
    }

    if USING_ZTS != 0 {
        println!("cargo:rustc-cfg=phper_zts");
    }
}
