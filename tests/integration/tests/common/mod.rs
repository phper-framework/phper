// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper_test::{cargo::CargoBuilder, fpm::FpmHandle};
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

#[allow(dead_code)]
pub(crate) static DYLIB_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let result = CargoBuilder::new()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .build()
        .unwrap();
    result.get_cdylib().unwrap()
});

#[allow(dead_code)]
pub(crate) static TESTS_PHP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("php")
});

#[allow(dead_code)]
pub(crate) static FPM_HANDLE: LazyLock<&FpmHandle> =
    LazyLock::new(|| FpmHandle::setup(&*DYLIB_PATH));
