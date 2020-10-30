use phper_macros::{c_str, c_str_ptr, php_fn, php_mn};
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

#[test]
fn test_php_fn() {
    let php_fn!(a): i32 = 1;
    assert_eq!(zif_a, 1);
}

#[test]
fn test_php_mn() {
    let php_mn!(a): i32 = 1;
    assert_eq!(zim_a, 1);
}
