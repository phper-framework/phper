#![feature(allocator_api)]

use std::alloc::{AllocRef, Layout, AllocError};
use std::ptr::{NonNull, slice_from_raw_parts_mut};
use phper_sys::{_emalloc, _efree};
use std::os::raw::c_char;

pub type EBox<T> = Box<T, Allocator>;

pub struct Allocator {
    #[cfg(phper_debug)]
    zend_filename: *const c_char,
    #[cfg(phper_debug)]
    zend_lineno: u32,
    #[cfg(phper_debug)]
    zend_orig_filename: *const c_char,
    #[cfg(phper_debug)]
    zend_orig_lineno: u32,
}

impl Allocator {
    pub const fn new(
        #[cfg(phper_debug)]
        zend_filename: *const c_char,
        #[cfg(phper_debug)]
        zend_lineno: u32,
        #[cfg(phper_debug)]
        zend_orig_filename: *const c_char,
        #[cfg(phper_debug)]
        zend_orig_lineno: u32,
    ) -> Self {
        Self {
            #[cfg(phper_debug)]
            zend_filename,
            #[cfg(phper_debug)]
            zend_lineno,
            #[cfg(phper_debug)]
            zend_orig_filename,
            #[cfg(phper_debug)]
            zend_orig_lineno,
        }
    }
}

unsafe impl AllocRef for Allocator {
    fn alloc(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            #[cfg(phper_debug)]
            let ptr = _emalloc(layout.size(), self.zend_filename, self.zend_lineno, self.zend_orig_filename, self.zend_orig_lineno);
            #[cfg(not(phper_debug))]
            let ptr = _emalloc(layout.size());

            if ptr.is_null() {
                Err(AllocError)
            } else {
                let ptr = slice_from_raw_parts_mut(ptr.cast(), layout.size());
                Ok(NonNull::new_unchecked(ptr))
            }
        }
    }

    unsafe fn dealloc(&self, ptr: NonNull<u8>, _layout: Layout) {
        // Not the correct position of `efree`, but can work!.
        #[cfg(phper_debug)]
        _efree(ptr.as_ptr().cast(), self.zend_filename, self.zend_lineno, self.zend_orig_filename, self.zend_orig_lineno);
        #[cfg(not(phper_debug))]
        _efree(ptr.as_ptr().cast());
    }
}
