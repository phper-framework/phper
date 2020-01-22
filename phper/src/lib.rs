pub extern crate phper_alloc as alloc;
extern crate phper_macros;
pub extern crate phper_sys as sys;

mod macros;

pub use phper_macros::*;
pub use phper_sys::c_str_ptr;
use sys::{zend_function_entry, zend_ini_entry_def, zend_module_entry};

pub mod zend;

mod function;
mod module;

pub use crate::function::*;
pub use crate::module::*;

pub type IniEntries = Vec<zend_ini_entry_def>;

pub type StaticZendModuleEntry = NotThreadSafe<*const zend_module_entry>;

pub type StaticZendFunctionEntry = NotThreadSafe<*const zend_function_entry>;

pub struct NotThreadSafe<T>(pub T);

unsafe impl<T> Sync for NotThreadSafe<T> {}
