// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_resource].

use crate::{sys::*, values::ZVal};

/// Wrapper of [crate::sys::zend_resource].
#[repr(transparent)]
pub struct ZRef {
    inner: zend_reference,
}

impl ZRef {
    /// # Safety
    ///
    /// Create from raw pointer.
    pub unsafe fn from_ptr<'a>(ptr: *const zend_reference) -> &'a Self {
        (ptr as *const Self)
            .as_ref()
            .expect("ptr should not be null")
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_reference) -> Option<&'a Self> {
        (ptr as *const Self).as_ref()
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_reference) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should not be null")
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_reference) -> Option<&'a mut Self> {
        (ptr as *mut Self).as_mut()
    }

    pub const fn as_ptr(&self) -> *const zend_reference {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_reference {
        &mut self.inner
    }

    pub fn val(&self) -> &ZVal {
        unsafe { ZVal::from_ptr(&self.inner.val) }
    }

    pub fn val_mut(&mut self) -> &mut ZVal {
        unsafe { ZVal::from_mut_ptr(&mut self.inner.val) }
    }
}
