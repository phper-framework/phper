// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_array].

use crate::{strings::ZStr, sys::*, values::ZVal};
use derive_more::From;
use phper_alloc::ToRefOwned;
use std::{
    borrow::Borrow,
    convert::TryInto,
    marker::PhantomData,
    mem::forget,
    ops::{Deref, DerefMut},
};

/// Key for [Array].
#[derive(Debug, Clone, PartialEq, From)]
pub enum Key<'a> {
    Index(u64),
    Str(&'a str),
    Bytes(&'a [u8]),
    ZStr(&'a ZStr),
}

/// Insert key for [Array].
#[derive(Debug, Clone, PartialEq, From)]
pub enum InsertKey<'a> {
    NextIndex,
    Index(u64),
    Str(&'a str),
    Bytes(&'a [u8]),
    ZStr(&'a ZStr),
}

#[repr(transparent)]
pub struct ZArr {
    inner: zend_array,
    _p: PhantomData<*mut ()>,
}

impl ZArr {
    pub unsafe fn from_ptr<'a>(ptr: *const zend_array) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_array) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    pub const fn as_ptr(&self) -> *const zend_array {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_array {
        &mut self.inner
    }

    #[inline]
    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }

    // Get items length.
    #[inline]
    pub fn len(&mut self) -> usize {
        unsafe { zend_array_count(self.as_mut_ptr()).try_into().unwrap() }
    }

    /// Add or update item by key.
    pub fn insert<'a>(&mut self, key: impl Into<InsertKey<'a>>, value: ZVal) {
        let key = key.into();
        unsafe {
            match key {
                InsertKey::NextIndex => {
                    phper_zend_hash_next_index_insert(self.as_mut_ptr(), value.into_raw());
                }
                InsertKey::Index(i) => {
                    phper_zend_hash_index_update(self.as_mut_ptr(), i, value.into_raw());
                }
                InsertKey::Str(s) => {
                    phper_zend_hash_str_update(
                        self.as_mut_ptr(),
                        s.as_ptr().cast(),
                        s.len().try_into().unwrap(),
                        value.into_raw(),
                    );
                }
                InsertKey::Bytes(b) => {
                    phper_zend_hash_str_update(
                        self.as_mut_ptr(),
                        b.as_ptr().cast(),
                        b.len().try_into().unwrap(),
                        value.into_raw(),
                    );
                }
                InsertKey::ZStr(s) => {
                    phper_zend_hash_str_update(
                        self.as_mut_ptr(),
                        s.as_c_str_ptr().cast(),
                        s.len().try_into().unwrap(),
                        value.into_raw(),
                    );
                }
            }
        }
    }

    // Get item by key.
    pub fn get<'a>(&self, key: impl Into<Key<'a>>) -> Option<&ZVal> {
        self.inner_get(key).map(|v| &*v)
    }

    // Get item by key.
    pub fn get_mut<'a>(&mut self, key: impl Into<Key<'a>>) -> Option<&mut ZVal> {
        self.inner_get(key)
    }

    fn inner_get<'a>(&self, key: impl Into<Key<'a>>) -> Option<&mut ZVal> {
        let key = key.into();
        unsafe {
            let value = match key {
                Key::Index(i) => zend_hash_index_find(self.as_ptr(), i),
                Key::Str(s) => zend_hash_str_find(
                    self.as_ptr(),
                    s.as_ptr().cast(),
                    s.len().try_into().unwrap(),
                ),
                Key::Bytes(b) => zend_hash_str_find(
                    self.as_ptr(),
                    b.as_ptr().cast(),
                    b.len().try_into().unwrap(),
                ),
                Key::ZStr(s) => {
                    zend_hash_str_find(self.as_ptr(), s.as_c_str_ptr(), s.len().try_into().unwrap())
                }
            };
            if value.is_null() {
                None
            } else {
                Some(ZVal::from_mut_ptr(value))
            }
        }
    }

    pub fn exists<'a>(&self, key: impl Into<Key<'a>>) -> bool {
        let key = key.into();
        unsafe {
            match key {
                Key::Index(i) => phper_zend_hash_index_exists(&self.inner, i),
                Key::Str(s) => phper_zend_hash_str_exists(
                    &self.inner,
                    s.as_ptr().cast(),
                    s.len().try_into().unwrap(),
                ),
                Key::Bytes(b) => phper_zend_hash_str_exists(
                    &self.inner,
                    b.as_ptr().cast(),
                    b.len().try_into().unwrap(),
                ),
                Key::ZStr(s) => phper_zend_hash_str_exists(
                    &self.inner,
                    s.to_bytes().as_ptr().cast(),
                    s.len().try_into().unwrap(),
                ),
            }
        }
    }

    pub fn remove<'a>(&mut self, key: impl Into<Key<'a>>) -> bool {
        let key = key.into();
        unsafe {
            (match key {
                Key::Index(i) => zend_hash_index_del(&mut self.inner, i),
                Key::Str(s) => zend_hash_str_del(
                    &mut self.inner,
                    s.as_ptr().cast(),
                    s.len().try_into().unwrap(),
                ),
                Key::Bytes(b) => zend_hash_str_del(
                    &mut self.inner,
                    b.as_ptr().cast(),
                    b.len().try_into().unwrap(),
                ),
                Key::ZStr(s) => zend_hash_str_del(
                    &mut self.inner,
                    s.as_c_str_ptr().cast(),
                    s.len().try_into().unwrap(),
                ),
            }) == ZEND_RESULT_CODE_SUCCESS
        }
    }

    pub fn clear(&mut self) {}

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            index: 0,
            array: self,
        }
    }

    pub fn entry<'a>(&mut self, key: impl Into<Key<'a>>) -> Entry<'a> {
        todo!()
    }
}

impl ToOwned for ZArr {
    type Owned = ZArray;

    fn to_owned(&self) -> Self::Owned {
        unsafe {
            // TODO The source really immutable?
            let dest = phper_zend_array_dup(self.as_ptr() as *mut _);
            ZArray::from_raw(dest)
        }
    }
}

impl ToRefOwned for ZArr {
    type Owned = ZArray;

    fn to_ref_owned(&mut self) -> Self::Owned {
        todo!()
    }
}

/// Wrapper of [crate::sys::zend_array].
#[repr(transparent)]
pub struct ZArray {
    inner: *mut ZArr,
}

impl ZArray {
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Note that the actual capacity is always a power of two, so if you have
    /// 12 elements in a hashtable the actual table capacity will be 16.
    pub fn with_capacity(n: usize) -> Self {
        unsafe {
            let ptr = phper_zend_new_array(n.try_into().unwrap());
            Self::from_raw(ptr)
        }
    }

    #[inline]
    pub unsafe fn from_raw(ptr: *mut zend_array) -> Self {
        Self {
            inner: ZArr::from_mut_ptr(ptr),
        }
    }

    #[inline]
    pub fn into_raw(mut self) -> *mut zend_array {
        let ptr = self.as_mut_ptr();
        forget(self);
        ptr
    }
}

impl Default for ZArray {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for ZArray {
    type Target = ZArr;

    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref().unwrap() }
    }
}

impl DerefMut for ZArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut().unwrap() }
    }
}

impl Borrow<ZArr> for ZArray {
    fn borrow(&self) -> &ZArr {
        self.deref()
    }
}

impl Clone for ZArray {
    fn clone(&self) -> Self {
        self.deref().to_owned()
    }
}

impl Drop for ZArray {
    fn drop(&mut self) {
        unsafe {
            zend_array_destroy(self.as_mut_ptr());
        }
    }
}

/// Iterator key for [Iter].
#[derive(Debug, Clone, PartialEq, From)]
pub enum IterKey<'a> {
    Index(u64),
    ZStr(&'a ZStr),
}

/// Iter created by [Array::iter].
pub struct Iter<'a> {
    index: isize,
    array: &'a ZArr,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (IterKey<'a>, &'a ZVal);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.array.inner.nNumUsed as isize {
                break None;
            }

            unsafe {
                let bucket = self.array.inner.arData.offset(self.index);

                let key = if (*bucket).key.is_null() {
                    IterKey::Index((*bucket).h)
                } else {
                    let s = ZStr::from_ptr((*bucket).key);
                    IterKey::ZStr(s)
                };

                let val = &mut (*bucket).val;
                let mut val = ZVal::from_mut_ptr(val);
                if val.get_type_info().is_indirect() {
                    val = ZVal::from_mut_ptr((*val.as_mut_ptr()).value.zv);
                }

                self.index += 1;

                if val.get_type_info().is_undef() {
                    continue;
                }
                break Some((key, val));
            }
        }
    }
}

// TODO Implement it.
pub enum Entry<'a> {
    Occupied(PhantomData<&'a ()>),
    Vacant(PhantomData<&'a ()>),
}

impl<'a> Entry<'a> {
    pub fn or_insert(&mut self, val: ZVal) -> &'a mut ZVal {
        todo!()
    }
}
