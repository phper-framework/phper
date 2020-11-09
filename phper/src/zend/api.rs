use crate::{
    sys::{zend_function_entry, zend_ini_entry_def},
    zend::ini::Mh,
};
use std::{
    cell::Cell,
    mem::{size_of, transmute},
    os::raw::c_int,
    ptr::null_mut,
};

pub const fn function_entry_end() -> zend_function_entry {
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
        zend_ini_entry_def {
            name: name.as_ptr().cast(),
            on_modify,
            mh_arg1: 0 as *mut _,
            mh_arg2: self.as_ptr().cast(),
            mh_arg3: null_mut(),
            value: default_value.as_ptr().cast(),
            displayer: None,
            modifiable: modifiable as c_int,
            name_length: name.len() as u32,
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

pub struct FunctionEntries<const N: usize> {
    inner: Cell<[zend_function_entry; N]>,
}

impl<const N: usize> FunctionEntries<N> {
    pub const fn new(inner: [zend_function_entry; N]) -> Self {
        Self {
            inner: Cell::new(inner),
        }
    }

    pub const fn as_ptr(&self) -> *mut zend_function_entry {
        self.inner.as_ptr().cast()
    }
}

unsafe impl<const N: usize> Sync for FunctionEntries<N> {}
