//! Apis relate to [crate::sys::zval].

use crate::{
    alloc::{EAllocatable, EBox},
    arrays::Array,
    classes::ClassEntry,
    errors::{NotRefCountedTypeError, Throwable, TypeError},
    functions::{call_internal, ZendFunction},
    objects::{Object, StatelessObject},
    strings::ZendString,
    sys::*,
    types::Type,
    utils::ensure_end_with_zero,
};
use indexmap::map::IndexMap;
use std::{
    collections::{BTreeMap, HashMap},
    mem::{transmute, zeroed},
    str,
    str::Utf8Error,
};

/// Wrapper of [crate::sys::zend_execute_data].
#[repr(transparent)]
pub struct ExecuteData {
    inner: zend_execute_data,
}

impl ExecuteData {
    #[inline]
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_execute_data) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_execute_data {
        &mut self.inner
    }

    #[inline]
    pub unsafe fn common_num_args(&self) -> u32 {
        (*self.inner.func).common.num_args
    }

    #[inline]
    pub unsafe fn common_required_num_args(&self) -> u16 {
        (*self.inner.func).common.required_num_args as u16
    }

    #[inline]
    pub unsafe fn common_arg_info(&self) -> *mut zend_arg_info {
        (*self.inner.func).common.arg_info
    }

    #[inline]
    pub unsafe fn num_args(&self) -> u16 {
        self.inner.This.u2.num_args as u16
    }

    pub unsafe fn func(&self) -> &ZendFunction {
        ZendFunction::from_mut_ptr(self.inner.func)
    }

    pub unsafe fn get_this<T>(&mut self) -> Option<&mut Object<T>> {
        let ptr = phper_get_this(&mut self.inner) as *mut Val;
        ptr.as_ref().map(|val| val.as_mut_object_unchecked())
    }

    /// TODO Do not return owned object, because usually Val should not be drop.
    pub(crate) unsafe fn get_parameters_array(&mut self) -> Vec<Val> {
        let num_args = self.num_args();
        let mut arguments = vec![zeroed::<zval>(); num_args as usize];
        if num_args > 0 {
            _zend_get_parameters_array_ex(num_args.into(), arguments.as_mut_ptr());
        }
        transmute(arguments)
    }
}

/// Wrapper of [crate::sys::zval].
///
/// TODO Refactor `as_*`, to `to_*` or return reference.
#[repr(transparent)]
pub struct Val {
    inner: zval,
}

impl Val {
    pub fn new(t: impl SetVal) -> Self {
        let mut val = unsafe { zeroed::<Val>() };
        unsafe {
            SetVal::set_val(t, &mut val);
        }
        val
    }

    pub fn undef() -> Self {
        let mut val = unsafe { zeroed::<Val>() };
        val.set_type(Type::undef());
        val
    }

    pub fn null() -> Self {
        Self::new(())
    }

    pub fn set(&mut self, t: impl SetVal) {
        unsafe {
            self.drop_value();
            SetVal::set_val(t, self);
        }
    }

    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zval) -> &'a mut Self {
        assert!(!ptr.is_null(), "ptr should not be null");
        &mut *(ptr as *mut Self)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const zval {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zval {
        &mut self.inner
    }

    pub fn get_type(&self) -> Type {
        let t = unsafe { self.inner.u1.type_info };
        t.into()
    }

    pub fn into_inner(self) -> zval {
        self.inner
    }

    fn get_type_name(&self) -> crate::Result<String> {
        self.get_type().get_base_type_name()
    }

    fn set_type(&mut self, t: Type) {
        self.inner.u1.type_info = t.into_raw();
    }

    pub fn as_null(&self) -> crate::Result<()> {
        if self.get_type().is_null() {
            Ok(())
        } else {
            Err(self.must_be_type_error("null").into())
        }
    }

    pub fn as_bool(&self) -> crate::Result<bool> {
        let t = self.get_type();
        if t.is_true() {
            Ok(true)
        } else if t.is_false() {
            Ok(false)
        } else {
            Err(self.must_be_type_error("bool").into())
        }
    }

    pub fn as_long(&self) -> crate::Result<i64> {
        if self.get_type().is_long() {
            unsafe { Ok(self.inner.value.lval) }
        } else {
            Err(self.must_be_type_error("int").into())
        }
    }

    pub fn as_long_value(&self) -> i64 {
        unsafe { phper_zval_get_long(&self.inner as *const _ as *mut _) }
    }

    pub fn as_double(&self) -> crate::Result<f64> {
        if self.get_type().is_double() {
            unsafe { Ok(self.inner.value.dval) }
        } else {
            Err(self.must_be_type_error("float").into())
        }
    }

    pub fn as_str(&self) -> crate::Result<&str> {
        Ok(self.as_zend_string()?.as_str()?)
    }

    pub fn as_string(&self) -> crate::Result<String> {
        if self.get_type().is_string() {
            unsafe {
                let zs = ZendString::from_ptr(self.inner.value.str).unwrap();
                Ok(zs.as_str()?.to_owned())
            }
        } else {
            Err(self.must_be_type_error("string").into())
        }
    }

    pub fn as_bytes(&self) -> crate::Result<&[u8]> {
        Ok(self.as_zend_string()?.as_ref())
    }

    pub fn as_zend_string(&self) -> crate::Result<&ZendString> {
        if self.get_type().is_string() {
            unsafe {
                let zs = ZendString::from_ptr(self.inner.value.str).unwrap();
                Ok(zs)
            }
        } else {
            Err(self.must_be_type_error("string").into())
        }
    }

    pub fn as_string_value(&self) -> Result<String, Utf8Error> {
        unsafe {
            let s = phper_zval_get_string(&self.inner as *const _ as *mut _);
            ZendString::from_raw(s).as_str().map(ToOwned::to_owned)
        }
    }

    pub fn as_array(&self) -> crate::Result<&Array> {
        if self.get_type().is_array() {
            unsafe {
                let ptr = self.inner.value.arr;
                Ok(Array::from_mut_ptr(ptr).unwrap())
            }
        } else {
            Err(self.must_be_type_error("array").into())
        }
    }

    pub fn as_object(&self) -> crate::Result<&Object<()>> {
        if self.get_type().is_object() {
            unsafe {
                let ptr = self.inner.value.obj;
                Ok(Object::from_mut_ptr(ptr))
            }
        } else {
            Err(self.must_be_type_error("object").into())
        }
    }

    pub(crate) unsafe fn as_mut_object_unchecked<T>(&self) -> &mut Object<T> {
        // TODO Fix the object type assertion.
        // assert!(dbg!(val.get_type()).is_object());

        let object = Object::from_mut_ptr(self.inner.value.obj);
        let class = object.get_class();
        ClassEntry::check_type_id(class).unwrap();
        object
    }

    // TODO Error tip, not only for function arguments, should change.
    fn must_be_type_error(&self, expect_type: &str) -> crate::Error {
        match self.get_type_name() {
            Ok(type_name) => {
                let message = format!("must be of type {}, {} given", expect_type, type_name);
                TypeError::new(message).into()
            }
            Err(e) => e.into(),
        }
    }

    /// Only add refcount.
    ///
    /// TODO Make a reference type to wrap self.
    pub fn duplicate(&mut self) -> Result<EBox<Self>, NotRefCountedTypeError> {
        unsafe {
            if !phper_z_refcounted_p(self.as_mut_ptr()) {
                Err(NotRefCountedTypeError)
            } else {
                (*self.inner.value.counted).gc.refcount += 1;
                let val = EBox::from_raw(self.as_mut_ptr().cast());
                Ok(val)
            }
        }
    }

    /// Call only when self is a callable.
    ///
    /// # Errors
    ///
    /// Return Err when self is not callable.
    pub fn call(&mut self, arguments: impl AsMut<[Val]>) -> crate::Result<EBox<Val>> {
        let none: Option<&mut StatelessObject> = None;
        call_internal(self, none, arguments)
    }

    unsafe fn drop_value(&mut self) {
        phper_zval_ptr_dtor_nogc(self.as_mut_ptr());
    }
}

impl EAllocatable for Val {
    fn free(ptr: *mut Self) {
        unsafe {
            ptr.as_mut().unwrap().drop_value();
            _efree(ptr.cast());
        }
    }
}

impl Drop for Val {
    fn drop(&mut self) {
        unsafe {
            self.drop_value();
        }
    }
}

/// The trait for setting the value of [Val], mainly as the return value of
/// functions and methods, and initializer of [Val].
///
/// TODO Better name, distinguish between non-referenced and referenced cases.
pub trait SetVal {
    unsafe fn set_val(self, val: &mut Val);
}

impl SetVal for () {
    unsafe fn set_val(self, val: &mut Val) {
        val.set_type(Type::null());
    }
}

impl SetVal for bool {
    unsafe fn set_val(self, val: &mut Val) {
        val.set_type(Type::bool(self));
    }
}

impl SetVal for i32 {
    unsafe fn set_val(self, val: &mut Val) {
        SetVal::set_val(self as i64, val)
    }
}

impl SetVal for u32 {
    unsafe fn set_val(self, val: &mut Val) {
        SetVal::set_val(self as i64, val)
    }
}

impl SetVal for i64 {
    unsafe fn set_val(self, val: &mut Val) {
        val.set_type(Type::long());
        (*val.as_mut_ptr()).value.lval = self;
    }
}

impl SetVal for f64 {
    unsafe fn set_val(self, val: &mut Val) {
        val.set_type(Type::double());
        (*val.as_mut_ptr()).value.dval = self;
    }
}

impl SetVal for &str {
    unsafe fn set_val(self, val: &mut Val) {
        phper_zval_stringl(val.as_mut_ptr(), self.as_ptr().cast(), self.len());
    }
}

impl SetVal for String {
    unsafe fn set_val(self, val: &mut Val) {
        let s: &str = &self;
        SetVal::set_val(s, val)
    }
}

impl SetVal for &[u8] {
    unsafe fn set_val(self, val: &mut Val) {
        // Because php string is binary safe, so can set `&[u8]` to php string.
        phper_zval_stringl(val.as_mut_ptr(), self.as_ptr().cast(), self.len());
    }
}

impl<const N: usize> SetVal for &[u8; N] {
    unsafe fn set_val(self, val: &mut Val) {
        // Because php string is binary safe, so can set `&[u8; N]` to php string.
        phper_zval_stringl(val.as_mut_ptr(), self.as_ptr().cast(), self.len());
    }
}

impl SetVal for Vec<u8> {
    unsafe fn set_val(self, val: &mut Val) {
        let v: &[u8] = &self;
        SetVal::set_val(v, val)
    }
}

impl<T: SetVal> SetVal for Vec<T> {
    unsafe fn set_val(self, val: &mut Val) {
        phper_array_init(val.as_mut_ptr());
        for (k, v) in self.into_iter().enumerate() {
            let v = EBox::new(Val::new(v));
            phper_zend_hash_index_update(
                (*val.as_mut_ptr()).value.arr,
                k as u64,
                EBox::into_raw(v).cast(),
            );
        }
    }
}

/// Setting the val to an array, Because of the feature of [std::collections::HashMap], the item
/// order of array is not guarantee.
impl<K: AsRef<str>, V: SetVal> SetVal for HashMap<K, V> {
    unsafe fn set_val(self, val: &mut Val) {
        map_set_val(self, val);
    }
}

/// Setting the val to an array, which preserves item order.
impl<K: AsRef<str>, V: SetVal> SetVal for IndexMap<K, V> {
    unsafe fn set_val(self, val: &mut Val) {
        map_set_val(self, val);
    }
}

impl<K: AsRef<str>, V: SetVal> SetVal for BTreeMap<K, V> {
    unsafe fn set_val(self, val: &mut Val) {
        map_set_val(self, val);
    }
}

unsafe fn map_set_val<K, V, I>(iterator: I, val: &mut Val)
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: SetVal,
{
    phper_array_init(val.as_mut_ptr());
    for (k, v) in iterator.into_iter() {
        let k = k.as_ref();
        let v = EBox::new(Val::new(v));
        phper_zend_hash_str_update(
            (*val.as_mut_ptr()).value.arr,
            k.as_ptr().cast(),
            k.len(),
            EBox::into_raw(v).cast(),
        );
    }
}

impl SetVal for EBox<Array> {
    unsafe fn set_val(self, val: &mut Val) {
        let arr = EBox::into_raw(self);
        phper_zval_arr(val.as_mut_ptr(), arr.cast());
    }
}

// TODO Support chain call for PHP object later, now only support return owned object.
impl<T> SetVal for EBox<Object<T>> {
    unsafe fn set_val(self, val: &mut Val) {
        let object = EBox::into_raw(self);
        val.inner.value.obj = object.cast();
        val.set_type(Type::object_ex());
    }
}

impl<T: SetVal> SetVal for Option<T> {
    unsafe fn set_val(self, val: &mut Val) {
        match self {
            Some(t) => SetVal::set_val(t, val),
            None => SetVal::set_val((), val),
        }
    }
}

impl<T: SetVal, E: Throwable> SetVal for Result<T, E> {
    unsafe fn set_val(self, val: &mut Val) {
        match self {
            Ok(t) => t.set_val(val),
            Err(e) => {
                let class_entry = e.class_entry();
                let message = ensure_end_with_zero(e.message());
                zend_throw_exception(
                    class_entry.as_ptr() as *mut _,
                    message.as_ptr().cast(),
                    e.code() as i64,
                );
                SetVal::set_val((), val);
            }
        }
    }
}

impl SetVal for Val {
    unsafe fn set_val(mut self, val: &mut Val) {
        phper_zval_copy(val.as_mut_ptr(), self.as_mut_ptr());
    }
}

impl SetVal for EBox<Val> {
    unsafe fn set_val(self, val: &mut Val) {
        phper_zval_zval(val.as_mut_ptr(), EBox::into_raw(self).cast(), 0, 1);
    }
}
