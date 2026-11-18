// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Internal crate that compiles the Rust code blocks of the documentation
//! under `docs/src` as doctests, so `cargo test` keeps them up to date.
//!
//! Every chapter must be registered in `chapters!` below; the
//! `every_chapter_is_doctested` test fails when one is missing.

#![warn(rust_2018_idioms, missing_docs)]

/// Includes each chapter as the documentation of an empty module, compiled
/// only for doctests, and records the chapter paths in `CHAPTERS`.
macro_rules! chapters {
    ($($(#[$attr:meta])* $name:ident => $path:literal,)*) => {
        $(
            #[cfg(doctest)]
            $(#[$attr])*
            #[doc = include_str!(concat!("../../../docs/src/", $path))]
            mod $name {}
        )*

        /// Chapter paths relative to `docs/src`.
        #[cfg(test)]
        const CHAPTERS: &[&str] = &[$($path),*];
    };
}

chapters! {
    introduction => "introduction.md",
    quick_start => "quick-start/index.md",
    write_your_first_extension => "quick-start/write-your-first-extension.md",
    write_a_simple_http_client => "quick-start/write-a-simple-http-client.md",
    integrate_with_pecl => "integrate-with-pecl.md",
    zval => "zval.md",
    internal_types => "internal-types/index.md",
    z_str => "internal-types/z-str.md",
    z_arr => "internal-types/z-arr.md",
    z_obj => "internal-types/z-obj.md",
    module => "module/index.md",
    hooks => "module/hooks.md",
    register_functions => "module/register-functions.md",
    register_constants => "module/register-constants.md",
    register_ini_settings => "module/register-ini-settings.md",
    extension_information => "module/extension-information.md",
    register_class => "module/register-class.md",
    register_interface => "module/register-interface.md",
    // Enums are only available since PHP 8.1.
    #[cfg(all(phper_major_version = "8", not(phper_minor_version = "0")))]
    register_enum => "module/register-enum.md",
}

#[cfg(test)]
mod tests {
    use super::CHAPTERS;
    use std::{
        collections::BTreeSet,
        fs,
        path::{Path, PathBuf},
    };

    fn docs_src() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/src")
    }

    /// Collects the paths of all markdown files under `dir`, relative to
    /// `root` and separated by `/`.
    fn collect_markdown(dir: &Path, root: &Path, out: &mut BTreeSet<String>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                collect_markdown(&path, root, out);
            } else if path.extension().is_some_and(|ext| ext == "md") {
                let relative = path
                    .strip_prefix(root)
                    .unwrap()
                    .components()
                    .map(|c| c.as_os_str().to_str().unwrap())
                    .collect::<Vec<_>>()
                    .join("/");
                out.insert(relative);
            }
        }
    }

    #[test]
    fn every_chapter_is_doctested() {
        let root = docs_src();
        let mut on_disk = BTreeSet::new();
        collect_markdown(&root, &root, &mut on_disk);
        on_disk.remove("SUMMARY.md");

        let registered = CHAPTERS
            .iter()
            .map(|path| path.to_string())
            .collect::<BTreeSet<_>>();

        let missing = on_disk.difference(&registered).collect::<Vec<_>>();
        let extra = registered.difference(&on_disk).collect::<Vec<_>>();
        assert!(
            missing.is_empty() && extra.is_empty(),
            "`chapters!` is out of sync with docs/src, not registered: {missing:?}, registered \
             but missing on disk: {extra:?}",
        );
    }
}
