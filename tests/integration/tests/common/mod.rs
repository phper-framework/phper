// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use env_logger::fmt::Formatter;
use log::kv::{self, Key, Value};
use phper_test::{cargo::CargoBuilder, fpm::FpmHandle};
use std::{
    path::{Path, PathBuf},
    sync::{LazyLock, Once},
};

pub static DYLIB_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let result = CargoBuilder::new()
        .arg("-j1")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .build()
        .unwrap();
    result.get_cdylib().unwrap()
});

pub static TESTS_PHP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("php")
});

#[allow(dead_code)]
pub static FPM_HANDLE: LazyLock<&FpmHandle> = LazyLock::new(|| FpmHandle::setup(&*DYLIB_PATH));

pub fn setup() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        env_logger::Builder::from_default_env()
            .default_format()
            .is_test(true)
            .format_key_values(|buf, args| {
                use std::io::Write as _;
                struct Visitor<'a>(&'a mut Formatter);
                impl<'kvs> kv::VisitSource<'kvs> for Visitor<'kvs> {
                    fn visit_pair(
                        &mut self, key: Key<'kvs>, value: Value<'kvs>,
                    ) -> Result<(), kv::Error> {
                        writeln!(self.0).unwrap();
                        writeln!(self.0, "===== {} =====", key).unwrap();
                        writeln!(self.0, "{}", value).unwrap();
                        Ok(())
                    }
                }
                args.visit(&mut Visitor(buf)).unwrap();
                Ok(())
            })
            .init();
    });
}
