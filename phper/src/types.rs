use crate::sys::*;
use std::{ffi::CStr, os::raw::c_int};

pub(crate) fn get_type_by_const(t: u32) -> crate::Result<String> {
    unsafe {
        let s = zend_get_type_by_const(t as c_int);
        let s = CStr::from_ptr(s).to_str()?.to_string();
        Ok(s)
    }
}
