// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_string].

use crate::{alloc::EBox, sys::*};
use std::{
    borrow::Borrow,
    convert::TryInto,
    marker::{PhantomData, PhantomPinned},
    mem::forget,
    ops::{Deref, DerefMut},
    os::raw::c_char,
    ptr::NonNull,
    slice::from_raw_parts,
    str,
    str::Utf8Error,
};

/// Like str, CStr for [crate::sys::zend_string].
#[repr(transparent)]
pub struct ZStr {
    inner: zend_string,
    _p: PhantomData<*mut ()>,
}

impl ZStr {
    pub unsafe fn from_ptr<'a>(ptr: *const zend_string) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_string) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    pub const fn as_ptr(&self) -> *const zend_string {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_string {
        &mut self.inner
    }

    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                phper_zstr_val(&self.inner).cast(),
                phper_zstr_len(&self.inner).try_into().unwrap(),
            )
        }
    }

    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.to_bytes())
    }
}

impl AsRef<[u8]> for ZStr {
    fn as_ref(&self) -> &[u8] {
        self.to_bytes()
    }
}

impl<Rhs: AsRef<[u8]>> PartialEq<Rhs> for ZStr {
    fn eq(&self, other: &Rhs) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl ToOwned for ZStr {
    type Owned = ZString;

    fn to_owned(&self) -> Self::Owned {
        ZString::new(self.to_bytes())
    }
}

/// Like String, CString for [crate::sys::zend_string].
pub struct ZString {
    inner: *mut ZStr,
}

impl ZString {
    pub fn new(s: impl AsRef<[u8]>) -> Self {
        unsafe {
            let s = s.as_ref();
            let ptr = phper_zend_string_init(
                s.as_ptr().cast(),
                s.len().try_into().unwrap(),
                false.into(),
            );
            Self::from_raw(ptr)
        }
    }

    #[inline]
    pub unsafe fn from_raw(ptr: *mut zend_string) -> Self {
        Self {
            inner: ZStr::from_mut_ptr(ptr),
        }
    }

    #[inline]
    pub fn into_raw(mut self) -> *mut zend_string {
        let ptr = self.as_mut_ptr();
        forget(self);
        ptr
    }
}

impl Clone for ZString {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = phper_zend_string_init(
                phper_zstr_val(self.as_ptr()),
                phper_zstr_len(self.as_ptr()).try_into().unwrap(),
                false.into(),
            );
            Self {
                inner: ZStr::from_mut_ptr(ptr.cast()),
            }
        }
    }
}

impl Deref for ZString {
    type Target = ZStr;

    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref().unwrap() }
    }
}

impl DerefMut for ZString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut().unwrap() }
    }
}

impl Borrow<ZStr> for ZString {
    fn borrow(&self) -> &ZStr {
        self.deref()
    }
}

impl AsRef<[u8]> for ZString {
    fn as_ref(&self) -> &[u8] {
        self.to_bytes()
    }
}

impl<Rhs: AsRef<[u8]>> PartialEq<Rhs> for ZString {
    fn eq(&self, other: &Rhs) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Drop for ZString {
    fn drop(&mut self) {
        unsafe {
            phper_zend_string_release(self.as_mut_ptr());
        }
    }
}
