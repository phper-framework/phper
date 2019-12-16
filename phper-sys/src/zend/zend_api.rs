#[macro_export]
macro_rules! zend_fe_end {
    () => {
        $crate::zend_function_entry {
            fname: 0 as *const c_char,
            handler: None,
            arg_info: 0 as *const $crate::zend_internal_arg_info,
            num_args: 0,
            flags: 0,
        }
    };
}
