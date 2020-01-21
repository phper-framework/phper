#[macro_export]
macro_rules! c_str {
    ($lit: expr) => {
        $crate::new_c_str_from_ptr_unchecked(
            ::std::concat!($lit, "\0").as_ptr() as *const ::std::os::raw::c_char
        )
    };
}

#[macro_export]
macro_rules! c_str_ptr {
    ($lit: expr) => {
        ::std::concat!($lit, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
}
