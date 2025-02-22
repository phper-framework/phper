// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![warn(rust_2018_idioms, missing_docs)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod macros;

use phper_sys::*;
use std::{
    borrow::Borrow,
    mem::{ManuallyDrop, size_of},
    ops::{Deref, DerefMut},
};

/// The Box which use php `emalloc` and `efree` to manage memory.
///
/// TODO Now feature `allocator_api` is still unstable, implement myself, use
/// Box<T, Alloc> later.
pub struct EBox<T> {
    ptr: *mut T,
}

impl<T> EBox<T> {
    /// Allocates heap memory using `emalloc` then places `x` into it.
    ///
    /// # Panic
    ///
    /// Panic if `size_of::<T>()` equals zero.
    #[allow(clippy::useless_conversion)]
    pub fn new(x: T) -> Self {
        unsafe {
            assert_ne!(size_of::<T>(), 0);
            let ptr: *mut T = phper_emalloc(size_of::<T>().try_into().unwrap()).cast();
            // TODO Deal with ptr is zero, when memory limit is reached.
            ptr.write(x);
            Self { ptr }
        }
    }

    /// Constructs from a raw pointer.
    ///
    /// # Safety
    ///
    /// Make sure the pointer is from `into_raw`, or created from `emalloc`.
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        Self { ptr: raw }
    }

    /// Consumes and returning a wrapped raw pointer.
    ///
    /// Will leak memory.
    pub fn into_raw(b: EBox<T>) -> *mut T {
        ManuallyDrop::new(b).ptr
    }

    /// Consumes the `EBox`, returning the wrapped value.
    pub fn into_inner(self) -> T {
        unsafe { self.ptr.read() }
    }
}

impl<T> Deref for EBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl<T> DerefMut for EBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut().unwrap() }
    }
}

impl<T> Drop for EBox<T> {
    fn drop(&mut self) {
        unsafe {
            self.ptr.drop_in_place();
            phper_efree(self.ptr.cast());
        }
    }
}

/// Duplicate an object without deep copy, but to only add the refcount, for php
/// refcount struct.
pub trait ToRefOwned {
    /// The resulting type after obtaining ownership.
    type Owned: Borrow<Self>;

    /// Creates owned data from borrowed data, by increasing refcount.
    fn to_ref_owned(&mut self) -> Self::Owned;
}

/// Duplicate an object without deep copy, but to only add the refcount, for php
/// refcount struct.
pub trait RefClone {
    /// Returns a refcount value with same reference of the value.
    fn ref_clone(&mut self) -> Self;
}
