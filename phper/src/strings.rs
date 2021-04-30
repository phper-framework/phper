use crate::sys::*;

#[repr(transparent)]
pub struct ZendString {
    inner: zend_string,
}
