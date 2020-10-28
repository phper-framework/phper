#![feature(allocator_api)]

use std::alloc::{AllocRef, Layout, AllocError};
use std::ptr::{NonNull, slice_from_raw_parts_mut};
use phper_sys::{_emalloc, _efree};

pub type EBox<T> = Box<T, Allocator>;

pub struct Allocator;

unsafe impl AllocRef for Allocator {
    fn alloc(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
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
        _efree(ptr.as_ptr().cast());
    }
}
