use crate::sys::*;
use num_traits::cast::FromPrimitive;
use std::{convert::TryInto, ffi::CStr, os::raw::c_int};

#[derive(FromPrimitive, PartialEq)]
#[repr(u32)]
#[non_exhaustive]
pub enum Type {
    Undef = IS_UNDEF,
    Null = IS_NULL,
    False = IS_FALSE,
    True = IS_TRUE,
    Long = IS_LONG,
    Double = IS_DOUBLE,
    String = IS_STRING,
    Array = IS_ARRAY,
    Object = IS_OBJECT,
    Resource = IS_RESOURCE,
    Reference = IS_REFERENCE,
    ConstantAst = IS_CONSTANT_AST,
    IsCallable = IS_CALLABLE,
}

impl From<u32> for Type {
    fn from(n: u32) -> Self {
        match FromPrimitive::from_u32(n) {
            Some(t) => t,
            None => unreachable!("Type is not exhaustive, should contains: {}", n),
        }
    }
}

pub(crate) fn get_type_by_const(t: u32) -> crate::Result<String> {
    unsafe {
        let s = zend_get_type_by_const(t as c_int);
        let s = CStr::from_ptr(s).to_str()?.to_string();
        Ok(s)
    }
}
