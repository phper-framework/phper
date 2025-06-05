// Copyright (c) 2025 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Memory allocation utilities and boxed types for PHP values.

pub use phper_alloc::{RefClone, ToRefOwned};
use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{self},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

/// A smart pointer for PHP values allocated in the Zend Engine memory.
///
/// `EBox<T>` provides owned access to values allocated in PHP's memory
/// management system. It automatically handles deallocation when dropped,
/// ensuring proper cleanup of PHP resources.
pub struct EBox<T> {
    ptr: *mut T,
}

impl<T> EBox<T> {
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
}

impl<T: fmt::Debug> fmt::Debug for EBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
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
        }
    }
}

impl<T> Borrow<T> for EBox<T> {
    fn borrow(&self) -> &T {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl<T> BorrowMut<T> for EBox<T> {
    fn borrow_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut().unwrap() }
    }
}

impl<T> AsRef<T> for EBox<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl<T> AsMut<T> for EBox<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut().unwrap() }
    }
}
