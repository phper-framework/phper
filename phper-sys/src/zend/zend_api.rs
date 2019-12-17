#[macro_export]
macro_rules! zend_fe_end {
    () => {
        $crate::zend_function_entry {
            fname: std::ptr::null(),
            handler: None,
            arg_info: std::ptr::null(),
            num_args: 0,
            flags: 0,
        }
    };
}
