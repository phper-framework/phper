use phper_macros::*;
use std::ffi::CStr;

#[test]
fn test_c_str() {
    assert_eq!(c_str!("foo"), unsafe {
        CStr::from_ptr("foo\0".as_ptr().cast())
    });
    assert_eq!(unsafe { c_str!("bar") }, unsafe {
        CStr::from_ptr("bar\0".as_ptr().cast())
    });
}

#[test]
fn test_c_str_ptr() {
    assert_eq!(c_str_ptr!("foo"), "foo\0".as_ptr().cast());
}
