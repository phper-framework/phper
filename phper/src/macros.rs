#[macro_export]
macro_rules! emalloc {
    ( $( $x:expr ),* ) => {
        ::phper_sys::_emalloc( $( $x, )* )
    };
}

#[macro_export]
macro_rules! efree {
    ( $( $x:expr ),* ) => {
        ::phper_sys::_efree( $( $x, )* )
    };
}

#[macro_export]
macro_rules! zend_call_num_args {
    ($execute_data: expr) => {
        (* $execute_data).This.u2.num_args
    };
}

#[macro_export]
macro_rules! zval_str {
    ($return_value: expr, $s: expr) => {
        let __z: *mut ::phper_sys::zval = $return_value;
        let __s: *mut ::phper_sys::zend_string = $s;
        $crate::z_str_p!(__z) = __s;
        $crate::z_type_info_p!(__z) = if $crate::zstr_is_interned!(__s) {
            ::phper_sys::IS_INTERNED_STRING_EX
         } else {
			::phper_sys::IS_STRING_EX
         } as u32;
    };
}

#[macro_export]
macro_rules! z_type_info_p {
    ($zval: expr) => {
        (*$zval).u1.type_info
    };
}

#[macro_export]
macro_rules! zstr_is_interned {
    ($s: expr) => {
        $crate::gc_flags!($s) as u32 & ::phper_sys::IS_STR_INTERNED != 0
    };
}

#[macro_export]
macro_rules! gc_flags {
    ($p: expr) => {
        (*$p).gc.u.v.flags
    };
}

#[macro_export]
macro_rules! z_str_p {
    ($zval_p: expr) => {
        $crate::z_str!(*$zval_p)
    };
}

#[macro_export]
macro_rules! z_str {
    ($zval: expr) => {
        ($zval).value.str
    };
}


#[macro_export]
macro_rules! php_fe_end {
    () => {
        ::phper_sys::zend_function_entry {
            fname: 0 as *const c_char,
            handler: None,
            arg_info: 0 as *const ::phper_sys::_zend_internal_arg_info,
            num_args: 0,
            flags: 0,
        }
    };
}


