#[macro_export]
macro_rules! gc_flags {
    ($p: expr) => {
        (*$p).gc.u.v.flags
    };
}

#[macro_export]
macro_rules! z_type_info {
    ($zval: expr) => {
        $zval.u1.type_info
    };
}

#[macro_export]
macro_rules! z_type_info_p {
    ($zval_p: expr) => {
        $crate::z_type_info!(*$zval_p)
    };
}

#[macro_export]
macro_rules! z_lval {
    ($zval: expr) => {
        $zval.value.lval
    };
}

#[macro_export]
macro_rules! z_lval_p {
    ($zval_p: expr) => {
        $crate::z_lval!(*$zval_p)
    };
}

#[macro_export]
macro_rules! z_dval {
    ($zval: expr) => {
        $zval.value.dval
    };
}

#[macro_export]
macro_rules! z_dval_p {
    ($zval_p: expr) => {
        $crate::z_dval!(*$zval_p)
    };
}

#[macro_export]
macro_rules! zval_null {
    ($z: expr) => {
        $crate::z_type_info!($z) = $crate::IS_NULL;
    };
}

#[macro_export]
macro_rules! zval_false {
    ($z: expr) => {
        $crate::z_type_info!($z) = $crate::IS_FALSE;
    };
}

#[macro_export]
macro_rules! zval_true {
    ($z: expr) => {
        $crate::z_type_info!($z) = $crate::IS_TRUE;
    };
}

#[macro_export]
macro_rules! zval_bool {
    ($z: expr, $b: expr) => {
        let $b: bool = $b;
        $crate::z_type_info!($z) = if $b { $crate::IS_TRUE } else { $crate::IS_FALSE };
    };
}

#[macro_export]
macro_rules! zval_long {
    ($z: expr, $l: expr) => {
    	let $z: *mut $crate::zval = $z;
        $crate::z_lval_p($z) = $l;
        $crate::z_type_info!($z) = $crate::IS_LONG;
    };
}

#[macro_export]
macro_rules! zval_double {
    ($z: expr, $d: expr) => {
    	let $z: *mut $crate::zval = $z;
        $crate::z_lval_p($z) = $l;
        $crate::z_type_info!($z) = $crate::IS_DOUBLE;
    };
}

#[macro_export]
macro_rules! z_str {
    ($zval: expr) => {
        ($zval).value.str
    };
}

#[macro_export]
macro_rules! z_str_p {
    ($zval_p: expr) => {
        $crate::z_str!(*$zval_p)
    };
}

#[macro_export]
macro_rules! zval_str {
    ($z: expr, $s: expr) => {
        let $z: *mut $crate::zval = $z;
        let $s: *mut $crate::zend_string = $s;
        $crate::z_str_p!($z) = $s;
        $crate::z_type_info_p!($z) = if $crate::zstr_is_interned!(__s) {
            ::phper_sys::IS_INTERNED_STRING_EX
         } else {
			::phper_sys::IS_STRING_EX
         } as u32;
    };
}

