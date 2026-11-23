#![warn(rust_2018_idioms, missing_docs)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/112468984?s=200&v=4")]

pub mod cargo;
pub mod cli;
mod context;
pub mod fpm;
pub mod log;
pub mod utils;
