#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]
/*!
Alloc related items for [phper](https://crates.io/crates/phper).

## License

[Unlicense](https://github.com/jmjoy/phper/blob/master/LICENSE).
*/

use phper_sys::*;
use std::{
    mem::{forget, size_of},
    ops::{Deref, DerefMut},
};

pub trait EAllocatable {
    fn free(ptr: *mut Self) {
        unsafe {
            _efree(ptr.cast());
        }
    }
}

/// The Box which use php `emalloc` and `efree` to manage memory.
///
/// TODO now feature `allocator_api` is still unstable, implement myself.
pub struct EBox<T: EAllocatable> {
    ptr: *mut T,
}

impl<T: EAllocatable> EBox<T> {
    pub fn new(x: T) -> Self {
        unsafe {
            let ptr: *mut T = _emalloc(size_of::<T>()).cast();
            ptr.write(x);
            Self { ptr }
        }
    }

    pub unsafe fn from_raw(raw: *mut T) -> Self {
        Self { ptr: raw }
    }

    pub fn into_raw(b: EBox<T>) -> *mut T {
        let ptr = b.ptr;
        forget(b);
        ptr
    }
}

impl<T: EAllocatable> Deref for EBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl<T: EAllocatable> DerefMut for EBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut().unwrap() }
    }
}

impl<T: EAllocatable> Drop for EBox<T> {
    fn drop(&mut self) {
        <T>::free(self.ptr);
    }
}
