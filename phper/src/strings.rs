// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_string].

use crate::sys::*;
use phper_alloc::ToRefOwned;
use std::{
    borrow::Borrow,
    convert::TryInto,
    ffi::{CStr, FromBytesWithNulError},
    fmt::Debug,
    marker::PhantomData,
    mem::forget,
    ops::{Deref, DerefMut},
    os::raw::c_char,
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
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const zend_string) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_string) -> Option<&'a Self> {
        (ptr as *const Self).as_ref()
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_string) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_string) -> Option<&'a mut Self> {
        (ptr as *mut Self).as_mut()
    }

    pub const fn as_ptr(&self) -> *const zend_string {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_string {
        &mut self.inner
    }

    #[inline]
    pub fn as_c_str_ptr(&self) -> *const c_char {
        unsafe { phper_zstr_val(&self.inner).cast() }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { phper_zstr_len(&self.inner).try_into().unwrap() }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(phper_zstr_val(&self.inner).cast(), self.len()) }
    }

    pub fn to_c_str(&self) -> Result<&CStr, FromBytesWithNulError> {
        CStr::from_bytes_with_nul(self.to_bytes())
    }

    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.to_bytes())
    }
}

impl Debug for ZStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ZStr").field(&self.to_c_str()).finish()
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

impl ToRefOwned for ZStr {
    type Owned = ZString;

    fn to_ref_owned(&mut self) -> Self::Owned {
        unsafe {
            let ptr = phper_zend_string_copy(self.as_mut_ptr());
            ZString::from_raw(ptr)
        }
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

    /// Create owned object From raw pointer, usually used in pairs with
    /// `into_raw`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to memory
    /// problems. For example, a double-free may occur if the function is called
    /// twice on the same raw pointer.
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

impl Debug for ZString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ZString").field(&self.to_c_str()).finish()
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
