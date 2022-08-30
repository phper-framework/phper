// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_array].

use crate::{alloc::ToRefOwned, strings::ZStr, sys::*, values::ZVal};
use derive_more::From;
use std::{
    borrow::Borrow,
    convert::TryInto,
    marker::PhantomData,
    mem::{forget, ManuallyDrop},
    ops::{Deref, DerefMut},
};

/// Key for [ZArr].
#[derive(Debug, Clone, PartialEq, From)]
pub enum Key<'a> {
    Index(u64),
    Str(&'a str),
    Bytes(&'a [u8]),
    ZStr(&'a ZStr),
}

/// Insert key for [ZArr].
#[derive(Debug, Clone, PartialEq, From)]
pub enum InsertKey<'a> {
    NextIndex,
    Index(u64),
    Str(&'a str),
    Bytes(&'a [u8]),
    ZStr(&'a ZStr),
}

impl<'a> From<Key<'a>> for InsertKey<'a> {
    fn from(k: Key<'a>) -> Self {
        match k {
            Key::Index(i) => InsertKey::Index(i),
            Key::Str(s) => InsertKey::Str(s),
            Key::Bytes(b) => InsertKey::Bytes(b),
            Key::ZStr(s) => InsertKey::ZStr(s),
        }
    }
}

#[repr(transparent)]
pub struct ZArr {
    inner: zend_array,
    _p: PhantomData<*mut ()>,
}

impl ZArr {
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const zend_array) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_array) -> Option<&'a Self> {
        (ptr as *const Self).as_ref()
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_array) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_array) -> Option<&'a mut Self> {
        (ptr as *mut Self).as_mut()
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
    pub fn insert<'a>(&mut self, key: impl Into<InsertKey<'a>>, mut value: ZVal) {
        let key = key.into();
        let val = value.as_mut_ptr();

        unsafe {
            match key {
                InsertKey::NextIndex => {
                    phper_zend_hash_next_index_insert(self.as_mut_ptr(), val);
                }
                InsertKey::Index(i) => {
                    phper_zend_hash_index_update(self.as_mut_ptr(), i, val);
                }
                InsertKey::Str(s) => {
                    phper_zend_symtable_str_update(
                        self.as_mut_ptr(),
                        s.as_ptr().cast(),
                        s.len().try_into().unwrap(),
                        val,
                    );
                }
                InsertKey::Bytes(b) => {
                    phper_zend_symtable_str_update(
                        self.as_mut_ptr(),
                        b.as_ptr().cast(),
                        b.len().try_into().unwrap(),
                        val,
                    );
                }
                InsertKey::ZStr(s) => {
                    phper_zend_symtable_str_update(
                        self.as_mut_ptr(),
                        s.as_c_str_ptr().cast(),
                        s.len().try_into().unwrap(),
                        val,
                    );
                }
            }
        }

        forget(value);
    }

    // Get item by key.
    pub fn get<'a>(&self, key: impl Into<Key<'a>>) -> Option<&'a ZVal> {
        self.inner_get(key).map(|v| &*v)
    }

    // Get item by key.
    pub fn get_mut<'a>(&mut self, key: impl Into<Key<'a>>) -> Option<&'a mut ZVal> {
        self.inner_get(key)
    }

    fn inner_get<'a>(&self, key: impl Into<Key<'a>>) -> Option<&'a mut ZVal> {
        let key = key.into();
        let ptr = self.as_ptr() as *mut _;
        unsafe {
            let value = match key {
                Key::Index(i) => phper_zend_hash_index_find(ptr, i),
                Key::Str(s) => phper_zend_symtable_str_find(
                    ptr,
                    s.as_ptr().cast(),
                    s.len().try_into().unwrap(),
                ),
                Key::Bytes(b) => phper_zend_symtable_str_find(
                    ptr,
                    b.as_ptr().cast(),
                    b.len().try_into().unwrap(),
                ),
                Key::ZStr(s) => {
                    phper_zend_symtable_str_find(ptr, s.as_c_str_ptr(), s.len().try_into().unwrap())
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
        let ptr = self.as_ptr() as *mut _;
        unsafe {
            match key {
                Key::Index(i) => phper_zend_hash_index_exists(ptr, i),
                Key::Str(s) => phper_zend_symtable_str_exists(
                    ptr,
                    s.as_ptr().cast(),
                    s.len().try_into().unwrap(),
                ),
                Key::Bytes(b) => phper_zend_symtable_str_exists(
                    ptr,
                    b.as_ptr().cast(),
                    b.len().try_into().unwrap(),
                ),
                Key::ZStr(s) => phper_zend_symtable_str_exists(
                    ptr,
                    s.to_bytes().as_ptr().cast(),
                    s.len().try_into().unwrap(),
                ),
            }
        }
    }

    pub fn remove<'a>(&mut self, key: impl Into<Key<'a>>) -> bool {
        let key = key.into();
        unsafe {
            match key {
                Key::Index(i) => phper_zend_hash_index_del(&mut self.inner, i),
                Key::Str(s) => phper_zend_symtable_str_del(
                    &mut self.inner,
                    s.as_ptr().cast(),
                    s.len().try_into().unwrap(),
                ),
                Key::Bytes(b) => phper_zend_symtable_str_del(
                    &mut self.inner,
                    b.as_ptr().cast(),
                    b.len().try_into().unwrap(),
                ),
                Key::ZStr(s) => phper_zend_symtable_str_del(
                    &mut self.inner,
                    s.as_c_str_ptr().cast(),
                    s.len().try_into().unwrap(),
                ),
            }
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            index: 0,
            array: self,
        }
    }

    pub fn entry<'a>(&'a mut self, key: impl Into<Key<'a>>) -> Entry<'a> {
        let key = key.into();
        match self.get_mut(key.clone()) {
            Some(val) => Entry::Occupied(val),
            None => Entry::Vacant { arr: self, key },
        }
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
        let mut val = ManuallyDrop::new(ZVal::default());
        unsafe {
            phper_zval_arr(val.as_mut_ptr(), self.as_mut_ptr());
            phper_z_addref_p(val.as_mut_ptr());
            ZArray::from_raw(val.as_mut_z_arr().unwrap().as_mut_ptr())
        }
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

    /// Create owned object From raw pointer, usually used in pairs with
    /// `into_raw`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to memory
    /// problems. For example, a double-free may occur if the function is called
    /// twice on the same raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut zend_array) -> Self {
        Self {
            inner: ZArr::from_mut_ptr(ptr),
        }
    }

    #[inline]
    pub fn into_raw(self) -> *mut zend_array {
        ManuallyDrop::new(self).as_mut_ptr()
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

/// Iter created by [ZArr::iter].
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

pub enum Entry<'a> {
    Occupied(&'a mut ZVal),
    Vacant { arr: &'a mut ZArr, key: Key<'a> },
}

impl<'a> Entry<'a> {
    pub fn or_insert(self, val: ZVal) -> &'a mut ZVal {
        match self {
            Entry::Occupied(val) => val,
            Entry::Vacant { arr, key } => {
                let insert_key: InsertKey<'_> = key.clone().into();
                arr.insert(insert_key, val);
                arr.get_mut(key).unwrap()
            }
        }
    }
}
