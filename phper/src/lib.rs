#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod macros;

pub mod arrays;
pub mod classes;
pub mod cmd;
pub mod errors;
pub mod exceptions;
pub mod functions;
pub mod ini;
pub mod modules;
pub mod objects;
pub mod output;
pub mod strings;
pub mod types;
mod utils;
pub mod values;

pub use crate::errors::{Error, Result};
pub use phper_alloc as alloc;
pub use phper_macros::*;
pub use phper_sys as sys;
