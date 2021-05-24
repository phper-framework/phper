//! Apis relate to [crate::sys::zend_array].

use crate::{
    alloc::{EAllocatable, EBox},
    strings::ZendString,
    sys::*,
    values::Val,
};
use std::{borrow::Cow, mem::zeroed};

/// Key for [Array].
#[derive(Debug, Clone, PartialEq)]
pub enum Key<'a> {
    Index(u64),
    Str(Cow<'a, str>),
}

impl From<u64> for Key<'_> {
    fn from(i: u64) -> Self {
        Key::Index(i)
    }
}

impl<'a> From<&'a str> for Key<'a> {
    fn from(s: &'a str) -> Self {
        Key::Str(Cow::Borrowed(s))
    }
}

impl From<String> for Key<'_> {
    fn from(s: String) -> Self {
        Key::Str(Cow::Owned(s))
    }
}

/// Wrapper of [crate::sys::zend_array].
#[repr(transparent)]
pub struct Array {
    inner: zend_array,
}

impl Array {
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

    pub(crate) unsafe fn from_mut_ptr<'a>(ptr: *mut zend_array) -> &'a mut Array {
        let ptr = ptr as *mut Array;
        ptr.as_mut().expect("ptr shouldn't be null")
    }

    pub fn as_ptr(&self) -> *const zend_array {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_array {
        &mut self.inner
    }

    // Add or update item by key.
    pub fn insert<'a>(&mut self, key: impl Into<Key<'a>>, value: Val) {
        let key = key.into();
        let value = EBox::new(value);
        unsafe {
            match key {
                Key::Index(i) => {
                    phper_zend_hash_index_update(&mut self.inner, i, EBox::into_raw(value).cast());
                }
                Key::Str(s) => {
                    phper_zend_hash_str_update(
                        &mut self.inner,
                        s.as_ptr().cast(),
                        s.len(),
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
                Key::Str(s) => zend_hash_str_find(&self.inner, s.as_ptr().cast(), s.len()),
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

    pub fn clone(&self) -> EBox<Self> {
        let mut other = Self::new();
        unsafe {
            zend_hash_copy(other.as_mut_ptr(), self.as_ptr() as *mut _, None);
        }
        other
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            index: 0,
            array: &self,
        }
    }
}

impl EAllocatable for Array {
    fn free(ptr: *mut Self) {
        unsafe {
            if (*ptr).inner.gc.refcount == 0 {
                zend_hash_destroy(ptr.cast());
                _efree(ptr.cast());
            }
        }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unreachable!("Allocation on the stack is not allowed")
    }
}

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
                    let s = ZendString::from_ptr((*bucket).key);
                    let s = s.to_string().unwrap();
                    Key::Str(Cow::Owned(s))
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
