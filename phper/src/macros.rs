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
    };
}


//		zval *__z = (z);						\
//		zend_string *__s = (s);					\
//		Z_STR_P(__z) = __s;						\
//		/* interned strings support */			\
//		Z_TYPE_INFO_P(__z) = ZSTR_IS_INTERNED(__s) ? \
//			IS_INTERNED_STRING_EX : 			\
//			IS_STRING_EX;						\




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



