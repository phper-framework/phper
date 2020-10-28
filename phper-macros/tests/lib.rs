use phper_macros::c_str;
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
