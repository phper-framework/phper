//! Apis relate to [crate::sys::zend_array].

use crate::{
    alloc::{EAllocatable, EBox},
    strings::ZendString,
    sys::*,
    values::Val,
};
use derive_more::From;
use std::{convert::TryInto, mem::zeroed};

/// Key for [Array].
#[derive(Debug, Clone, PartialEq, From)]
pub enum Key<'a> {
    Index(u64),
    Str(&'a str),
}

/// Insert key for [Array].
#[derive(Debug, Clone, PartialEq, From)]
pub enum InsertKey<'a> {
    NextIndex,
    Index(u64),
    Str(&'a str),
}

/// Wrapper of [crate::sys::zend_array].
#[repr(transparent)]
pub struct Array {
    inner: zend_array,
}

impl Array {
    #[allow(clippy::useless_conversion)]
    pub fn new() -> EBox<Self> {
        unsafe {
            let mut array = EBox::new(zeroed::<Array>());
            _zend_hash_init(
                array.as_mut_ptr(),
                0,
                Some(phper_zval_ptr_dtor),
                false.into(),
            );
            array
        }
    }

    /// New Array reference from raw pointer.
    ///
    /// # Safety
    ///
    /// Make sure pointer is the type of `zend_array'.
    pub unsafe fn from_ptr<'a>(ptr: *const zend_array) -> Option<&'a Array> {
        let ptr = ptr as *const Array;
        ptr.as_ref()
    }

    /// New Array mutable reference from raw pointer.
    ///
    /// # Safety
    ///
    /// Make sure pointer is the type of `zend_array'.
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_array) -> Option<&'a mut Array> {
        let ptr = ptr as *mut Array;
        ptr.as_mut()
    }

    pub fn as_ptr(&self) -> *const zend_array {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_array {
        &mut self.inner
    }

    /// Add or update item by key.
    pub fn insert<'a>(&mut self, key: impl Into<InsertKey<'a>>, value: Val) {
        let key = key.into();
        let value = EBox::new(value);
        unsafe {
            match key {
                InsertKey::NextIndex => {
                    phper_zend_hash_next_index_insert(
                        &mut self.inner,
                        EBox::into_raw(value).cast(),
                    );
                }
                InsertKey::Index(i) => {
                    phper_zend_hash_index_update(&mut self.inner, i, EBox::into_raw(value).cast());
                }
                InsertKey::Str(s) => {
                    phper_zend_hash_str_update(
                        &mut self.inner,
                        s.as_ptr().cast(),
                        s.len().try_into().unwrap(),
                        EBox::into_raw(value).cast(),
                    );
                }
            }
        }
    }

    // Get item by key.
    pub fn get<'a>(&self, key: impl Into<Key<'a>>) -> Option<&Val> {
        let key = key.into();
        unsafe {
            let value = match key {
                Key::Index(i) => zend_hash_index_find(&self.inner, i),
                Key::Str(s) => {
                    zend_hash_str_find(&self.inner, s.as_ptr().cast(), s.len().try_into().unwrap())
                }
            };
            if value.is_null() {
                None
            } else {
                Some(Val::from_mut_ptr(value))
            }
        }
    }

    // Get item by key.
    pub fn get_mut<'a>(&mut self, key: impl Into<Key<'a>>) -> Option<&mut Val> {
        let key = key.into();
        unsafe {
            let value = match key {
                Key::Index(i) => zend_hash_index_find(&self.inner, i),
                Key::Str(s) => {
                    zend_hash_str_find(&self.inner, s.as_ptr().cast(), s.len().try_into().unwrap())
                }
            };
            if value.is_null() {
                None
            } else {
                Some(Val::from_mut_ptr(value))
            }
        }
    }

    // Get items length.
    pub fn len(&mut self) -> usize {
        unsafe { zend_array_count(&mut self.inner) as usize }
    }

    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
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
            }) == ZEND_RESULT_CODE_SUCCESS
        }
    }

    pub fn clone_arr(&self) -> EBox<Self> {
        let mut other = Self::new();
        unsafe {
            zend_hash_copy(other.as_mut_ptr(), self.as_ptr() as *mut _, None);
        }
        other
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            index: 0,
            array: self,
        }
    }
}

impl EAllocatable for Array {
    unsafe fn free(ptr: *mut Self) {
        (*ptr).inner.gc.refcount -= 1;
        if (*ptr).inner.gc.refcount == 0 {
            zend_array_destroy(ptr.cast());
        }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unreachable!("Allocation on the stack is not allowed")
    }
}

/// Iter created by [Array::iter].
pub struct Iter<'a> {
    index: isize,
    array: &'a Array,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Key<'a>, &'a Val);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.array.inner.nNumUsed as isize {
                break None;
            }

            unsafe {
                let bucket = self.array.inner.arData.offset(self.index);

                let key = if (*bucket).key.is_null() {
                    Key::Index((*bucket).h)
                } else {
                    let s = ZendString::from_ptr((*bucket).key).unwrap();
                    let s = s.as_str().unwrap();
                    Key::Str(s)
                };

                let val = &mut (*bucket).val;
                let mut val = Val::from_mut_ptr(val);
                if val.get_type().is_indirect() {
                    val = Val::from_mut_ptr((*val.as_mut_ptr()).value.zv);
                }

                self.index += 1;

                if val.get_type().is_undef() {
                    continue;
                }
                break Some((key, val));
            }
        }
    }
}
