use crate::zend::errors::Level;
use std::{os::raw::c_int, ptr::null};

pub fn error_doc_ref(level: Level, message: impl ToString) {
    let mut message = message.to_string();
    message.push('\0');

    unsafe {
        #[cfg(phper_php_version = "7.4")]
        crate::sys::php_error_docref(null(), level as c_int, message.as_ptr().cast());

        #[cfg(any(
            phper_php_version = "7.3",
            phper_php_version = "7.2",
            phper_php_version = "7.1",
            phper_php_version = "7.0",
        ))]
        crate::sys::php_error_docref0(null(), level as c_int, message.as_ptr().cast());
    }
}
