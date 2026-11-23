//! Internal useful utils.

use std::ffi::CString;

pub(crate) fn ensure_end_with_zero(s: impl Into<String>) -> CString {
    CString::new(s.into()).expect("CString::new failed")
}
