// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_resource].

use crate::sys::*;
use std::fmt::{self, Debug};

/// Wrapper of [zend_resource].
#[repr(transparent)]
pub struct ZRes {
    inner: zend_resource,
}

impl ZRes {
    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    pub unsafe fn from_ptr<'a>(ptr: *const zend_resource) -> &'a Self {
        unsafe {
            (ptr as *const Self)
                .as_ref()
                .expect("ptr should not be null")
        }
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_resource) -> Option<&'a Self> {
        unsafe { (ptr as *const Self).as_ref() }
    }

    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_resource) -> &'a mut Self {
        unsafe { (ptr as *mut Self).as_mut().expect("ptr should not be null") }
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_resource) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const zend_resource {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_resource {
        &mut self.inner
    }

    /// Gets the resource handle.
    #[allow(clippy::useless_conversion)]
    pub fn handle(&self) -> i64 {
        self.inner.handle.into()
    }
}

impl Debug for ZRes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZRes")
            .field("handle", &self.handle())
            .finish()
    }
}
