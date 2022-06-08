// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]

use phper_sys::*;
use std::{
    convert::TryInto,
    mem::{forget, size_of},
    ops::{Deref, DerefMut},
};

// TODO Add ERc, for refcounted type.

/// The item which can be placed into container [EBox].
pub trait EAllocatable {
    /// The method to free the heap allocated by `emalloc`, should call `efree`
    /// at the end.
    ///
    /// # Safety
    unsafe fn free(ptr: *mut Self) {
        _efree(ptr.cast());
    }
}

/// The Box which use php `emalloc` and `efree` to manage memory.
///
/// TODO now feature `allocator_api` is still unstable, implement myself, use
/// Box<T, Alloc> later.
pub struct EBox<T: EAllocatable> {
    ptr: *mut T,
}

impl<T: EAllocatable> EBox<T> {
    /// Allocates heap memory using `emalloc` then places `x` into it.
    ///
    /// # Panic
    ///
    /// Panic if `size_of::<T>()` equals zero.
    pub fn new(x: T) -> Self {
        unsafe {
            assert_ne!(size_of::<T>(), 0);
            let ptr: *mut T = _emalloc(size_of::<T>().try_into().unwrap()).cast();
            ptr.write(x);
            Self { ptr }
        }
    }

    /// Constructs from a raw pointer.
    ///
    /// # Safety
    ///
    /// Make sure the pointer is created from `emalloc`.
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        Self { ptr: raw }
    }

    /// Consumes and returning a wrapped raw pointer.
    ///
    /// Will leak memory.
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
        unsafe {
            <T>::free(self.ptr);
        }
    }
}

unsafe impl<T: EAllocatable> Send for EBox<T> {}

// TODO Write Erc for gc_refcounted holding types.
// pub trait ERcAble {
//     // Increment the reference count;
//     fn incr(&mut self);
//
//     /// Decrement the reference count and return old count.
//     fn decr(&mut self) -> usize;
// }
//
// pub struct ERc<T> {
//     value: T,
// }
//
// impl<T> ERc<T> {
//     pub fn new(value: T) -> Self {
//         Self { value }
//     }
// }
