// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_string].

use crate::{alloc::EBox, sys::*};
use phper_alloc::ToRefOwned;
use std::{
    borrow::Cow,
    ffi::{CStr, FromBytesWithNulError},
    fmt::{self, Debug},
    marker::PhantomData,
    os::raw::c_char,
    slice::from_raw_parts,
    str::{self, Utf8Error},
};

/// Like str, CStr for [zend_string].
#[repr(transparent)]
pub struct ZStr {
    inner: zend_string,
    _p: PhantomData<*mut ()>,
}

impl ZStr {
    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const zend_string) -> &'a Self {
        unsafe { (ptr as *const Self).as_ref().expect("ptr should't be null") }
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_string) -> Option<&'a Self> {
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
    #[inline]
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_string) -> &'a mut Self {
        unsafe { (ptr as *mut Self).as_mut().expect("ptr should't be null") }
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_string) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const zend_string {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_string {
        &mut self.inner
    }

    /// Converts to a raw C string pointer.
    #[inline]
    pub fn as_c_str_ptr(&self) -> *const c_char {
        unsafe { phper_zstr_val(&self.inner).cast() }
    }

    /// Gets the inner C string length.
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { phper_zstr_len(&self.inner).try_into().unwrap() }
    }

    /// Returns `true` if `self` has a length of zero bytes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Converts inner C string to a byte slice.
    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(phper_zstr_val(&self.inner).cast(), self.len()) }
    }

    /// Converts inner C string to a byte slice containing the trailing 0 byte..
    #[inline]
    fn to_bytes_with_nul(&self) -> &[u8] {
        unsafe { from_raw_parts(phper_zstr_val(&self.inner).cast(), self.len() + 1) }
    }

    /// Extracts a [`CStr`] slice containing the inner C string.
    pub fn to_c_str(&self) -> Result<&CStr, FromBytesWithNulError> {
        CStr::from_bytes_with_nul(self.to_bytes_with_nul())
    }

    /// Yields a str slice if the `ZStr` contains valid UTF-8.
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.to_bytes())
    }

    /// Converts a slice of bytes to a string, including invalid characters.
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.to_bytes())
    }
}

impl Drop for ZStr {
    fn drop(&mut self) {
        unsafe {
            phper_zend_string_release(self.as_mut_ptr());
        }
    }
}

impl Debug for ZStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        common_fmt(self, f, "ZStr")
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
            ZString::from_raw_cast(ptr)
        }
    }
}

/// An owned PHP string value.
///
/// `ZString` represents an owned PHP string (zend_string) allocated in the Zend
/// Engine memory. It provides safe access to PHP string operations and
/// automatically manages memory cleanup using reference counting.
pub type ZString = EBox<ZStr>;

impl ZString {
    /// Creates a new zend string from a container of bytes.
    #[allow(clippy::useless_conversion)]
    pub fn new(s: impl AsRef<[u8]>) -> Self {
        unsafe {
            let s = s.as_ref();
            let ptr = phper_zend_string_init(
                s.as_ptr().cast(),
                s.len().try_into().unwrap(),
                false.into(),
            );
            Self::from_raw_cast(ptr)
        }
    }

    /// Creates a new persistent zend string from a container of bytes.
    #[allow(clippy::useless_conversion)]
    pub fn new_persistent(s: impl AsRef<[u8]>) -> Self {
        unsafe {
            let s = s.as_ref();
            let ptr =
                phper_zend_string_init(s.as_ptr().cast(), s.len().try_into().unwrap(), true.into());
            Self::from_raw_cast(ptr)
        }
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
            Self::from_raw_cast(ZStr::from_mut_ptr(ptr))
        }
    }
}

impl AsRef<[u8]> for ZString {
    fn as_ref(&self) -> &[u8] {
        self.to_bytes()
    }
}

impl<Rhs: AsRef<[u8]>> PartialEq<Rhs> for ZString {
    fn eq(&self, other: &Rhs) -> bool {
        AsRef::<[u8]>::as_ref(self) == other.as_ref()
    }
}

fn common_fmt(this: &ZStr, f: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    let mut d = f.debug_tuple(name);
    match this.to_c_str() {
        Ok(s) => {
            d.field(&s);
        }
        Err(e) => {
            d.field(&e);
        }
    };
    d.finish()
}
