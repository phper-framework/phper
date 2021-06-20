#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/*!
Integration test tool for [phper](https://crates.io/crates/phper).

The `php-config` is needed. You can set environment `PHP_CONFIG` to specify the path.

## License

[Unlicense](https://github.com/jmjoy/phper/blob/master/LICENSE).
!*/

use crate::context::Context;
use std::{
    panic::{catch_unwind, resume_unwind, UnwindSafe},
    path::Path,
    process::{Child, Output},
};

pub mod cli;
mod context;
#[cfg(feature = "fpm")]
#[cfg_attr(docsrs, doc(cfg(feature = "fpm")))]
pub mod fpm;
mod utils;
