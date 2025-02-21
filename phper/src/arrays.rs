// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_array].

use crate::{alloc::ToRefOwned, strings::ZStr, sys::*, values::ZVal};
use derive_more::From;
use std::{
    borrow::Borrow,
    fmt::{self, Debug},
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

/// Key for [ZArr].
#[derive(Debug, Clone, PartialEq, From)]
pub enum Key<'a> {
    /// Index type key.
    Index(u64),
    /// String type key.
    Str(&'a str),
    /// String type key.
    Bytes(&'a [u8]),
    /// String type key.
    ZStr(&'a ZStr),
}

/// Insert key for [ZArr].
#[derive(Debug, Clone, PartialEq, From)]
pub enum InsertKey<'a> {
    /// Insert with next index type key, like `$arr[] = "foo"` in PHP.
    NextIndex,
    /// Insert with index type key.
    Index(u64),
    /// Insert with string type key.
    Str(&'a str),
    /// Insert with string type key.
    Bytes(&'a [u8]),
    /// Insert with string type key.
    ZStr(&'a ZStr),
}

impl From<()> for InsertKey<'_> {
    fn from(_: ()) -> Self {
        Self::NextIndex
    }
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

/// Wrapper of [zend_array].
#[repr(transparent)]
pub struct ZArr {
    inner: zend_array,
    _p: PhantomData<*mut ()>,
}

impl ZArr {
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
    pub unsafe fn from_ptr<'a>(ptr: *const zend_array) -> &'a Self {
        unsafe { (ptr as *const Self).as_ref().expect("ptr should't be null") }
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_array) -> Option<&'a Self> {
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
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_array) -> &'a mut Self {
        unsafe { (ptr as *mut Self).as_mut().expect("ptr should't be null") }
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_array) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const zend_array {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_array {
        &mut self.inner
    }

    /// Returns true if the array has a length of 0.
    #[inline]
    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }

    /// Get array items length.
    #[inline]
    pub fn len(&mut self) -> usize {
        unsafe { zend_array_count(self.as_mut_ptr()).try_into().unwrap() }
    }

    /// Add or update item by key.
    ///
    /// Notice that phper prefer to use [`Symtables`](https://www.phpinternalsbook.com/php5/hashtables/array_api.html#symtables) api `zend_symtable_*`,
    /// so `insert(42)` and `insert("42")` should be considered the same.
    #[allow(clippy::useless_conversion)]
    pub fn insert<'a>(&mut self, key: impl Into<InsertKey<'a>>, value: impl Into<ZVal>) {
        let key = key.into();
        let mut value = ManuallyDrop::new(value.into());
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
    }

    /// Get item by key.
    ///
    /// Notice that phper prefer to use [`Symtables`](https://www.phpinternalsbook.com/php5/hashtables/array_api.html#symtables) api `zend_symtable_*`,
    /// so `get(42)` and `get("42")` should be considered the same.
    pub fn get<'a>(&self, key: impl Into<Key<'a>>) -> Option<&'a ZVal> {
        self.inner_get(key).map(|v| &*v)
    }

    /// Get item by key.
    ///
    /// Notice that phper prefer to use [`Symtables`](https://www.phpinternalsbook.com/php5/hashtables/array_api.html#symtables) api `zend_symtable_*`,
    /// so `get_mut(42)` and `get_mut("42")` should be considered the same.
    pub fn get_mut<'a>(&mut self, key: impl Into<Key<'a>>) -> Option<&'a mut ZVal> {
        self.inner_get(key)
    }

    #[allow(clippy::useless_conversion)]
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

    /// Check if the key exists.
    ///
    /// Notice that phper prefer to use [`Symtables`](https://www.phpinternalsbook.com/php5/hashtables/array_api.html#symtables) api `zend_symtable_*`,
    /// so `exists(42)` and `exists("42")` should be considered the same.
    #[allow(clippy::useless_conversion)]
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

    /// Remove the item under the key
    ///
    /// Notice that phper prefer to use [`Symtables`](https://www.phpinternalsbook.com/php5/hashtables/array_api.html#symtables) api `zend_symtable_*`,
    /// so `remove(42)` and `remove("42")` should be considered the same.
    #[allow(clippy::useless_conversion)]
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

    /// Gets the given key’s corresponding entry in the array for in-place
    /// manipulation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::arrays::ZArray;
    ///
    /// let mut arr = ZArray::new();
    ///
    /// // count the number of occurrences of letters in the vec
    /// for x in ["a", "b", "a", "c", "a", "b"] {
    ///     arr.entry(x)
    ///         .and_modify(|cur| *cur.as_mut_long().unwrap() += 1)
    ///         .or_insert(1);
    /// }
    /// ```
    pub fn entry<'a>(&'a mut self, key: impl Into<Key<'a>>) -> Entry<'a> {
        let key = key.into();
        match self.get_mut(key.clone()) {
            Some(val) => Entry::Occupied(OccupiedEntry(val)),
            None => Entry::Vacant(VacantEntry { arr: self, key }),
        }
    }

    /// Provides a forward iterator.
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }

    /// Provides a forward iterator with mutable references.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut::new(self)
    }
}

impl Debug for ZArr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        common_fmt(self, f, "ZArr")
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

/// Wrapper of [zend_array].
#[repr(transparent)]
pub struct ZArray {
    inner: *mut ZArr,
}

impl ZArray {
    /// Creates an empty `ZArray`.
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates an empty `ZArray` with at least the specified capacity.
    ///
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
        unsafe {
            Self {
                inner: ZArr::from_mut_ptr(ptr),
            }
        }
    }

    /// Consumes the `ZArray` and transfers ownership to a raw pointer.
    ///
    /// Failure to call [`ZArray::from_raw`] will lead to a memory leak.
    #[inline]
    pub fn into_raw(self) -> *mut zend_array {
        ManuallyDrop::new(self).as_mut_ptr()
    }
}

impl Debug for ZArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        common_fmt(self, f, "ZArray")
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

/// Iterator key for [`ZArr::iter`] and [`ZArr::iter_mut`].
#[derive(Debug, Clone, PartialEq, From)]
pub enum IterKey<'a> {
    /// Index type iterator key.
    Index(u64),
    /// String type iterator key.
    ZStr(&'a ZStr),
}

struct RawIter<'a> {
    arr: *mut zend_array,
    pos: HashPosition,
    finished: bool,
    _p: PhantomData<&'a ()>,
}

impl RawIter<'_> {
    fn new(arr: *mut zend_array) -> Self {
        let mut pos: HashPosition = 0;
        unsafe {
            zend_hash_internal_pointer_reset_ex(arr, &mut pos);
        }
        Self {
            arr,
            pos,
            finished: false,
            _p: PhantomData,
        }
    }
}

impl<'a> Iterator for RawIter<'a> {
    type Item = (IterKey<'a>, *mut zval);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.finished {
                return None;
            }

            let mut str_index: *mut zend_string = null_mut();
            let mut num_index: zend_ulong = 0;

            #[allow(clippy::unnecessary_mut_passed)]
            let result = zend_hash_get_current_key_ex(
                self.arr,
                &mut str_index,
                &mut num_index,
                &mut self.pos,
            ) as u32;

            let iter_key = if result == HASH_KEY_IS_STRING {
                IterKey::ZStr(ZStr::from_mut_ptr(str_index))
            } else if result == HASH_KEY_IS_LONG {
                #[allow(clippy::unnecessary_cast)]
                IterKey::Index(num_index as u64)
            } else {
                self.finished = true;
                return None;
            };

            let val = zend_hash_get_current_data_ex(self.arr, &mut self.pos);
            if val.is_null() {
                self.finished = true;
                return None;
            }

            if zend_hash_move_forward_ex(self.arr, &mut self.pos) == ZEND_RESULT_CODE_FAILURE {
                self.finished = true;
            }

            Some((iter_key, val))
        }
    }
}

/// An iterator over the elements of a `ZArr`.
///
/// This is created by [`iter`].
///
/// [`iter`]: ZArr::iter
pub struct Iter<'a>(RawIter<'a>);

impl<'a> Iter<'a> {
    fn new(arr: &'a ZArr) -> Self {
        Self(RawIter::new(arr.as_ptr() as *mut _))
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (IterKey<'a>, &'a ZVal);

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(key, val)| (key, unsafe { ZVal::from_ptr(val) }))
    }
}

/// An mutable iterator over the elements of a `ZArr`.
///
/// This is created by [`iter_mut`].
///
/// [`iter_mut`]: ZArr::iter_mut
pub struct IterMut<'a>(RawIter<'a>);

impl<'a> IterMut<'a> {
    fn new(arr: &'a mut ZArr) -> Self {
        Self(RawIter::new(arr.as_mut_ptr()))
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (IterKey<'a>, &'a mut ZVal);

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(key, val)| (key, unsafe { ZVal::from_mut_ptr(val) }))
    }
}

/// A view into a single entry in an array, which may either be vacant or
/// occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`ZArr`].
///
/// [`entry`]: ZArr::entry
pub enum Entry<'a> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a>),
    /// A vacant entry.
    Vacant(VacantEntry<'a>),
}

/// A view into an occupied entry in a `ZArr`.
/// It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a>(&'a mut ZVal);

/// A view into a vacant entry in a `ZArr`.
/// It is part of the [`Entry`] enum.
pub struct VacantEntry<'a> {
    arr: &'a mut ZArr,
    key: Key<'a>,
}

impl<'a> Entry<'a> {
    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the array.
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut ZVal),
    {
        match self {
            Entry::Occupied(entry) => {
                f(entry.0);
                Entry::Occupied(entry)
            }
            entry => entry,
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty, and
    /// returns a mutable reference to the value in the entry.
    pub fn or_insert(self, val: impl Into<ZVal>) -> &'a mut ZVal {
        match self {
            Entry::Occupied(entry) => entry.0,
            Entry::Vacant(entry) => {
                let insert_key: InsertKey<'_> = entry.key.clone().into();
                entry.arr.insert(insert_key, val);
                entry.arr.get_mut(entry.key).unwrap()
            }
        }
    }
}

fn common_fmt(this: &ZArr, f: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    struct Debugger<'a>(&'a ZArr);

    impl Debug for Debugger<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_map().entries(self.0.iter()).finish()
        }
    }

    let zd = Debugger(this);

    f.debug_tuple(name).field(&zd).finish()
}
