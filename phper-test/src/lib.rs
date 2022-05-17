#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub mod cli;
mod context;
#[cfg(feature = "fpm")]
#[cfg_attr(docsrs, doc(cfg(feature = "fpm")))]
pub mod fpm;
mod utils;
