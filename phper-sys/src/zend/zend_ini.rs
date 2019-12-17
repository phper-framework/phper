#[macro_export]
macro_rules! zend_ini_end {
    () => {
        $crate::zend_ini_entry_def {
            name: std::ptr::null_mut(),
            on_modify: None,
            mh_arg1: std::ptr::null_mut(),
            mh_arg2: std::ptr::null_mut(),
            mh_arg3: std::ptr::null_mut(),
            value: std::ptr::null_mut(),
            displayer: None,
            modifiable: 0,
            name_length: 0,
            value_length: 0,
        }
    };
}

