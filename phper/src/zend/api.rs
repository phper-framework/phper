use crate::{
    sys::{zend_function_entry, zend_ini_entry_def},
    zend::ini::Mh,
};
use std::{
    cell::Cell,
    mem::{size_of, transmute},
    ptr::null_mut,
};

const fn function_entry_end() -> zend_function_entry {
    unsafe { transmute([0u8; size_of::<zend_function_entry>()]) }
}

pub struct ModuleGlobals<T: 'static> {
    inner: Cell<T>,
}

impl<T: 'static> ModuleGlobals<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner: Cell::new(inner),
        }
    }

    pub const fn as_ptr(&self) -> *mut T {
        self.inner.as_ptr()
    }

    pub const fn create_ini_entry_def(
        &'static self,
        name: &str,
        default_value: &str,
        on_modify: Option<Mh>,
        modifiable: u32,
    ) -> zend_ini_entry_def {
        #[cfg(any(phper_php_version = "7.4", phper_php_version = "7.3",))]
        let (modifiable, name_length) = (modifiable as std::os::raw::c_uchar, name.len() as u16);
        #[cfg(any(
            phper_php_version = "7.2",
            phper_php_version = "7.1",
            phper_php_version = "7.0",
        ))]
        let (modifiable, name_length) = (modifiable as std::os::raw::c_int, name.len() as u32);

        zend_ini_entry_def {
            name: name.as_ptr().cast(),
            on_modify,
            mh_arg1: 0 as *mut _,
            mh_arg2: self.as_ptr().cast(),
            mh_arg3: null_mut(),
            value: default_value.as_ptr().cast(),
            displayer: None,
            modifiable,
            name_length,
            value_length: default_value.len() as u32,
        }
    }
}

impl<T: Copy + 'static> ModuleGlobals<T> {
    pub fn get(&self) -> T {
        self.inner.get()
    }
}

unsafe impl<T: 'static> Sync for ModuleGlobals<T> {}

#[repr(C)]
struct ZendFunctionEntriesWithEnd<const N: usize>([zend_function_entry; N], zend_function_entry);

pub struct FunctionEntries<const N: usize> {
    inner: Cell<ZendFunctionEntriesWithEnd<N>>,
}

impl<const N: usize> FunctionEntries<N> {
    pub const fn new(inner: [zend_function_entry; N]) -> Self {
        Self {
            inner: Cell::new(ZendFunctionEntriesWithEnd(inner, function_entry_end())),
        }
    }

    pub const fn as_ptr(&self) -> *mut zend_function_entry {
        self.inner.as_ptr().cast()
    }
}

unsafe impl<const N: usize> Sync for FunctionEntries<N> {}
