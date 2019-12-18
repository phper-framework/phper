#[macro_export]
macro_rules! c_str_ptr {
    ($lit: expr) => {
        ::std::concat!($lit, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
}

#[macro_export]
macro_rules! php_ini {
    ($($x:expr),*) => {
        vec![
            $($x,)*
            ::phper_sys::zend_ini_end!()
        ]
    };
    ($($x:expr,)*) => {
        $crate::php_ini![$($x),*]
    };
}

#[macro_export]
macro_rules! std_php_ini_entry {
    ($name:expr,$default:expr,$modifiable:expr,$on_modify:expr,$arg2:expr) => {
        {
            let name: &'static str = ::std::concat!($name, "\0");
            let name_len = name.len() - 1;

            let value: &'static str = ::std::concat!($default, "\0");
            let value_len = value.len() - 1;

            let arg2 = $arg2.with(|i| i.as_ptr() as *mut c_void);

            ::phper_sys::zend_ini_entry_def {
                name: name.as_ptr() as *const c_char,
                on_modify: Some($on_modify),
                mh_arg1: ::std::ptr::null_mut(),
                mh_arg2: arg2,
                mh_arg3: ::std::ptr::null_mut(),
                value: value.as_ptr() as *const c_char,
                displayer: None,
                modifiable: $modifiable as c_int,
                name_length: name_len as c_uint,
                value_length: value_len as c_uint,
            }
        }
    };
}

