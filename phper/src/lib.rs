extern crate phper_macros;

pub mod alloc;
pub mod context;
mod macros;

pub use phper_macros::*;
use std::cell::Cell;
use phper_sys::zend_ini_entry_def;

pub type IniEntries = Vec<zend_ini_entry_def>;

pub struct NotThreadSafe<T>(pub T);

unsafe impl<T> Sync for NotThreadSafe<T> {}
