use crate::{
    c_str_ptr,
    sys::{
        self, phper_get_this, phper_init_class_entry, phper_z_strval_p, phper_zval_get_type,
        phper_zval_stringl, zend_class_entry, zend_declare_property_stringl, zend_execute_data,
        zend_parse_parameters, zend_read_property, zend_register_internal_class,
        zend_throw_exception, zval, IS_FALSE, IS_LONG, IS_NULL, IS_TRUE, ZEND_RESULT_CODE_SUCCESS,
    },
    zend::{api::FunctionEntries, exceptions::Throwable},
};
use std::{
    borrow::Cow,
    cell::Cell,
    ffi::{c_void, CStr},
    os::raw::{c_char, c_int},
    ptr::null_mut,
};

pub struct ClassEntry {
    inner: Cell<*mut zend_class_entry>,
}

impl ClassEntry {
    pub const fn new() -> Self {
        Self {
            inner: Cell::new(null_mut()),
        }
    }

    pub const fn as_ptr(&self) -> *mut *mut zend_class_entry {
        self.inner.as_ptr()
    }

    pub fn get(&self) -> *mut zend_class_entry {
        self.inner.get()
    }

    pub fn init<const N: usize>(&self, class_name: *const c_char, functions: &FunctionEntries<N>) {
        unsafe {
            let mut class_ce = phper_init_class_entry(class_name, functions.as_ptr());
            *self.as_ptr() = zend_register_internal_class(&mut class_ce);
        }
    }

    pub fn declare_property(
        &self,
        name: impl AsRef<str>,
        value: impl DeclareProperty,
        access_type: c_int,
    ) -> bool {
        unsafe {
            let name = name.as_ref();
            value.declare_property(self.get(), name, access_type) == ZEND_RESULT_CODE_SUCCESS
        }
    }

    pub fn read_property(&self, this: *mut zval, name: impl AsRef<str>) -> &mut Val {
        let name = name.as_ref();
        unsafe {
            let v = zend_read_property(
                self.get(),
                this,
                name.as_ptr().cast(),
                name.len(),
                1,
                null_mut(),
            );
            Val::from_mut(v)
        }
    }
}

unsafe impl Sync for ClassEntry {}

pub trait DeclareProperty {
    unsafe fn declare_property(
        self,
        ce: *mut zend_class_entry,
        name: &str,
        access_type: c_int,
    ) -> c_int;
}

impl DeclareProperty for &str {
    unsafe fn declare_property(
        self,
        ce: *mut zend_class_entry,
        name: &str,
        access_type: i32,
    ) -> c_int {
        zend_declare_property_stringl(
            ce,
            name.as_ptr().cast(),
            name.len(),
            self.as_ptr().cast(),
            self.len(),
            access_type,
        )
    }
}

#[repr(transparent)]
pub struct ExecuteData {
    inner: zend_execute_data,
}

impl ExecuteData {
    pub unsafe fn from_mut<'a>(ptr: *mut zend_execute_data) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }

    pub fn as_mut(&mut self) -> *mut zend_execute_data {
        &mut self.inner
    }

    #[inline]
    pub fn num_args(&self) -> usize {
        unsafe { self.inner.This.u2.num_args as usize }
    }

    #[inline]
    pub fn get_this(&mut self) -> *mut zval {
        unsafe { phper_get_this(&mut self.inner) }
    }

    pub fn parse_parameters<T: ParseParameter>(&self) -> Option<T> {
        <T>::parse(self.num_args())
    }
}

pub trait ParseParameter: Sized {
    fn spec() -> Cow<'static, str>;

    fn num_parameters() -> usize;

    fn parameters() -> Vec<*mut c_void>;

    fn from_parameters(parameters: &[*mut c_void]) -> Option<Self>;

    fn parse(num_args: usize) -> Option<Self> {
        let parameters = Self::parameters();
        if zend_parse_fixed_parameters(num_args, &Self::spec(), &parameters) {
            Self::from_parameters(&parameters)
        } else {
            None
        }
    }
}

impl ParseParameter for () {
    fn spec() -> Cow<'static, str> {
        Cow::Borrowed("")
    }

    fn num_parameters() -> usize {
        0
    }

    fn parameters() -> Vec<*mut c_void> {
        Vec::new()
    }

    fn from_parameters(_parameters: &[*mut c_void]) -> Option<Self> {
        Some(())
    }
}

impl ParseParameter for bool {
    #[inline]
    fn spec() -> Cow<'static, str> {
        Cow::Borrowed("b")
    }

    #[inline]
    fn num_parameters() -> usize {
        1
    }

    #[inline]
    fn parameters() -> Vec<*mut c_void> {
        vec![Box::into_raw(Box::new(false)).cast()]
    }

    fn from_parameters(parameters: &[*mut c_void]) -> Option<Self> {
        let b = unsafe { Box::from_raw(parameters[0] as *mut bool) };
        Some(*b)
    }
}

impl ParseParameter for i64 {
    #[inline]
    fn spec() -> Cow<'static, str> {
        Cow::Borrowed("l")
    }

    #[inline]
    fn num_parameters() -> usize {
        1
    }

    #[inline]
    fn parameters() -> Vec<*mut c_void> {
        vec![Box::into_raw(Box::new(0i64)).cast()]
    }

    fn from_parameters(parameters: &[*mut c_void]) -> Option<Self> {
        let i = unsafe { Box::from_raw(parameters[0] as *mut i64) };
        Some(*i)
    }
}

impl ParseParameter for f64 {
    #[inline]
    fn spec() -> Cow<'static, str> {
        Cow::Borrowed("d")
    }

    #[inline]
    fn num_parameters() -> usize {
        1
    }

    #[inline]
    fn parameters() -> Vec<*mut c_void> {
        vec![Box::into_raw(Box::new(0f64)).cast()]
    }

    fn from_parameters(parameters: &[*mut c_void]) -> Option<Self> {
        let i = unsafe { Box::from_raw(parameters[0] as *mut f64) };
        Some(*i)
    }
}

impl ParseParameter for &str {
    #[inline]
    fn spec() -> Cow<'static, str> {
        Cow::Borrowed("s")
    }

    #[inline]
    fn num_parameters() -> usize {
        2
    }

    #[inline]
    fn parameters() -> Vec<*mut c_void> {
        vec![
            Box::into_raw(Box::new(null_mut::<c_char>())).cast(),
            Box::into_raw(Box::new(0u32)).cast(),
        ]
    }

    fn from_parameters(parameters: &[*mut c_void]) -> Option<Self> {
        unsafe {
            let ptr = Box::from_raw(parameters[0] as *mut *mut c_char);
            let _len = Box::from_raw(parameters[1] as *mut c_int);
            CStr::from_ptr(*ptr).to_str().ok()
        }
    }
}

macro_rules! impl_parse_parameter_for_tuple {
    ( $(($t:ident,$T:ident)),* ) => {
        impl<$($T: ParseParameter,)*> ParseParameter for ($($T,)*) {
            fn spec() -> Cow<'static, str> {
                let mut s= String::new();
                $(s.push_str(&<$T>::spec());)*
                Cow::Owned(s)
            }

            #[inline]
            fn num_parameters() -> usize {
                0 $( + <$T>::num_parameters())*
            }

            fn parameters() -> Vec<*mut c_void> {
                let mut parameters = Vec::new();
                $(parameters.extend_from_slice(&<$T>::parameters());)*
                parameters
            }

            fn from_parameters(parameters: &[*mut c_void]) -> Option<Self> {
                let mut i = 0;

                $(let $t = {
                    let j = i;
                    i += <$T>::num_parameters();
                    match <$T>::from_parameters(&parameters[j..i]) {
                        Some(item) => item,
                        None => return None,
                    }
                };)*

                Some(($($t,)*))
            }
        }
    };
}

#[rustfmt::skip] impl_parse_parameter_for_tuple!((a, A));
#[rustfmt::skip] impl_parse_parameter_for_tuple!((a, A), (b, B));
#[rustfmt::skip] impl_parse_parameter_for_tuple!((a, A), (b, B), (c, C));
#[rustfmt::skip] impl_parse_parameter_for_tuple!((a, A), (b, B), (c, C), (d, D));
#[rustfmt::skip] impl_parse_parameter_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E));
#[rustfmt::skip] impl_parse_parameter_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F));
#[rustfmt::skip] impl_parse_parameter_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F), (g, G));
#[rustfmt::skip] impl_parse_parameter_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F), (g, G), (h, H));
#[rustfmt::skip] impl_parse_parameter_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F), (g, G), (h, H), (i, I));
#[rustfmt::skip] impl_parse_parameter_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F), (g, G), (h, H), (i, I), (j, J));

macro_rules! call_zend_parse_parameters {
    ( $num_args:expr, $type_spec:expr, $parameters:expr $(,$i:expr)* ) => {
        unsafe { zend_parse_parameters($num_args, $type_spec, $($parameters.get_unchecked($i).clone(),)*) }
    }
}

fn zend_parse_fixed_parameters(
    num_args: usize,
    type_spec: &str,
    parameters: &[*mut c_void],
) -> bool {
    assert!(parameters.len() <= 20);
    let type_spec = format!("{}\0", type_spec);

    #[rustfmt::skip]
    let b = match parameters.len() {
        0  => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters),
        1  => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0),
        2  => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1),
        3  => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2),
        4  => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3),
        5  => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4),
        6  => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5),
        7  => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6),
        8  => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7),
        9  => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8),
        10 => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
        11 => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10),
        12 => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),
        13 => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
        14 => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13),
        15 => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14),
        16 => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
        17 => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16),
        18 => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17),
        19 => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18),
        20 => call_zend_parse_parameters!(num_args as c_int, type_spec.as_ptr().cast(), parameters, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19),
        _  => unreachable!(),
    };

    b == ZEND_RESULT_CODE_SUCCESS
}

#[repr(transparent)]
pub struct Val {
    inner: zval,
}

impl Val {
    pub unsafe fn from_mut<'a>(ptr: *mut zval) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }

    pub fn as_mut(&mut self) -> *mut zval {
        &mut self.inner
    }

    pub fn try_into_value<'a>(&self) -> crate::Result<Value<'a>> {
        Value::from_ptr(&self.inner)
    }

    unsafe fn type_info(&mut self) -> &mut u32 {
        &mut self.inner.u1.type_info
    }
}

pub trait SetVal {
    fn set_val(self, val: &mut Val);
}

impl SetVal for () {
    fn set_val(self, val: &mut Val) {
        unsafe {
            *val.type_info() = IS_NULL;
        }
    }
}

impl SetVal for bool {
    fn set_val(self, val: &mut Val) {
        unsafe {
            *val.type_info() = if self { IS_TRUE } else { IS_FALSE };
        }
    }
}

impl SetVal for i32 {
    fn set_val(self, val: &mut Val) {
        (self as i64).set_val(val)
    }
}

impl SetVal for u32 {
    fn set_val(self, val: &mut Val) {
        (self as i64).set_val(val)
    }
}

impl SetVal for i64 {
    fn set_val(self, val: &mut Val) {
        unsafe {
            (*val.as_mut()).value.lval = self;
            (*val.as_mut()).u1.type_info = IS_LONG;
        }
    }
}

impl SetVal for &str {
    fn set_val(self, val: &mut Val) {
        unsafe {
            phper_zval_stringl(val.as_mut(), self.as_ptr().cast(), self.len());
        }
    }
}

impl SetVal for String {
    fn set_val(self, val: &mut Val) {
        unsafe {
            phper_zval_stringl(val.as_mut(), self.as_ptr().cast(), self.len());
        }
    }
}

impl<T: SetVal> SetVal for Option<T> {
    fn set_val(self, val: &mut Val) {
        match self {
            Some(t) => t.set_val(val),
            None => ().set_val(val),
        }
    }
}

impl<T: SetVal, E: Throwable> SetVal for Result<T, E> {
    fn set_val(self, val: &mut Val) {
        match self {
            Ok(t) => t.set_val(val),
            Err(_e) => unsafe {
                zend_throw_exception(null_mut(), c_str_ptr!(""), 0);
                todo!();
            },
        }
    }
}

#[derive(Debug)]
pub enum Value<'a> {
    Null,
    Bool(bool),
    Long(i64),
    Double(f64),
    CStr(&'a CStr),
    Array(()),
    Object(()),
    Resource(()),
}

impl<'a> Value<'a> {
    pub fn from_ptr(v: *const zval) -> crate::Result<Self> {
        unsafe {
            match phper_zval_get_type(v) as u32 {
                sys::IS_NULL => Ok(Self::Null),
                sys::IS_FALSE => Ok(Self::Bool(false)),
                sys::IS_TRUE => Ok(Self::Bool(true)),
                sys::IS_LONG => Ok(Self::Long((*v).value.lval)),
                sys::IS_DOUBLE => Ok(Self::Double((*v).value.dval)),
                sys::IS_STRING | sys::IS_STRING_EX => {
                    let s = phper_z_strval_p(v);
                    Ok(Self::CStr(CStr::from_ptr(s)))
                }
                t => Err(crate::Error::UnKnownValueType(t)),
            }
        }
    }
}
