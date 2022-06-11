// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_resource].

use crate::sys::*;

/// Wrapper of [crate::sys::zend_resource].
#[repr(transparent)]
pub struct ZRes {
    inner: zend_resource,
}

impl ZRes {
    pub unsafe fn from_ptr<'a>(ptr: *const zend_resource) -> &'a Self {
        (ptr as *const Self)
            .as_ref()
            .expect("ptr should not be null")
    }

    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_resource) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should not be null")
    }

    #[allow(clippy::useless_conversion)]
    pub fn handle(&self) -> i64 {
        self.inner.handle.into()
    }
}
