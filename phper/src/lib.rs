extern crate phper_macros;

mod macros;

pub use phper_macros::*;
pub use phper_sys as sys;
pub use phper_sys::c_str_ptr;
use sys::{zend_function_entry, zend_ini_entry_def, zend_module_entry};

pub type IniEntries = Vec<zend_ini_entry_def>;

pub type StaticZendModuleEntry = NotThreadSafe<*const zend_module_entry>;

pub type StaticZendFunctionEntry = NotThreadSafe<*const zend_function_entry>;

pub struct NotThreadSafe<T>(pub T);

unsafe impl<T> Sync for NotThreadSafe<T> {}
