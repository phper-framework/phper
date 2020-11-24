use crate::{
    sys::{zend_function_entry, zend_ini_entry_def, zend_internal_arg_info, zif_handler},
    zend::{
        compile::MultiInternalArgInfo,
        ini::{create_ini_entry_ex, Mh},
    },
};
use std::{
    cell::Cell,
    mem::{size_of, transmute},
    os::raw::c_char,
    ptr::null,
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

    pub const fn create_ini_entry(
        &self,
        name: &str,
        default_value: &str,
        on_modify: Option<Mh>,
        modifiable: u32,
    ) -> zend_ini_entry_def {
        create_ini_entry_ex(
            name,
            default_value,
            on_modify,
            modifiable,
            self.as_ptr().cast(),
        )
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

pub struct FunctionEntryBuilder {
    fname: *const c_char,
    handler: zif_handler,
    arg_info: *const zend_internal_arg_info,
    num_args: u32,
    flags: u32,
}

impl FunctionEntryBuilder {
    pub const fn new(fname: *const c_char, handler: zif_handler) -> Self {
        Self {
            fname,
            handler,
            arg_info: null(),
            num_args: 0,
            flags: 0,
        }
    }

    pub const fn arg_info<const N: usize>(
        self,
        arg_info: &'static MultiInternalArgInfo<N>,
    ) -> Self {
        Self {
            arg_info: arg_info.as_ptr(),
            num_args: arg_info.len() as u32,
            ..self
        }
    }

    pub const fn flags(self, flags: u32) -> Self {
        Self { flags, ..self }
    }

    pub const fn build(self) -> zend_function_entry {
        zend_function_entry {
            fname: self.fname,
            handler: self.handler,
            arg_info: self.arg_info,
            num_args: self.num_args,
            flags: self.flags,
        }
    }
}
