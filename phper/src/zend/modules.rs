use crate::sys::zend_module_entry;
use std::cell::UnsafeCell;

pub struct ModuleEntry {
    raw: UnsafeCell<zend_module_entry>,
}

impl ModuleEntry {
    pub const fn new(raw: zend_module_entry) -> Self {
        Self { raw: UnsafeCell::new(raw) }
    }

    #[inline]
    pub fn get(&self) -> *mut zend_module_entry {
        self.raw.get()
    }
}

unsafe impl Sync for ModuleEntry {}
