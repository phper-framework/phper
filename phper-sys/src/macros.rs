#[macro_export]
macro_rules! c_str_ptr {
    ($lit: expr) => {
        ::std::concat!($lit, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
}
