#[macro_export]
macro_rules! emalloc {
    ($size: expr) => {
        $crate::sys::_emalloc($size)
    };
}

#[macro_export]
macro_rules! efree {
    ($ptr: expr) => {
        $crate::sys::_efree($ptr)
    };
}

#[macro_export]
macro_rules! zend_call_num_args {
    ($call: expr) => {
        (*$call).This.u2.num_args
    };
}

#[macro_export]
macro_rules! define_php_ini {
    ($($x:expr),*) => {
        ::std::vec![
            $($x,)*
            $crate::zend_ini_end!()
        ]
    };
    ($($x:expr,)*) => {
        $crate::define_php_ini![$($x),*]
    };
}

#[macro_export]
macro_rules! zend_ini_end {
    () => {
        $crate::sys::zend_ini_entry_def {
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

#[macro_export]
macro_rules! std_php_ini_entry {
    ($name:expr,$default:expr,$modifiable:expr,$on_modify:expr,$arg2:expr) => {{
        let name: &'static str = ::std::concat!($name, "\0");
        let name_len = name.len() - 1;

        let value: &'static str = ::std::concat!($default, "\0");
        let value_len = value.len() - 1;

        let arg2 = $arg2.with(|i| i.as_ptr() as *mut c_void);

        $crate::sys::zend_ini_entry_def {
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
    }};
}

#[macro_export]
macro_rules! define_zend_functions {
    ($($x:expr),*) => {
        $crate::NotThreadSafe(&[
            $($x,)*
            $crate::zend_fe_end!(),
        ] as *const $crate::sys::_zend_function_entry);
    };
    ($($x:expr,)*) => {
        $crate::define_zend_functions![$($x),*]
    };
}

#[macro_export]
macro_rules! zend_fe_end {
    () => {
        $crate::sys::zend_function_entry {
            fname: std::ptr::null(),
            handler: None,
            arg_info: std::ptr::null(),
            num_args: 0,
            flags: 0,
        }
    };
}

#[macro_export]
macro_rules! zend_function_entry {
    ($fname:expr,$handler:expr) => {
        $crate::zend_function_entry!($fname, $handler, ::std::ptr::null(), 0, 0)
    };
    ($fname:expr,$handler:expr,$arg_info:expr,$num_args:expr) => {
        $crate::zend_function_entry!($fname, $handler, $arg_info, $num_args, 0)
    };
    ($fname:expr,$handler:expr,$arg_info:expr,$num_args:expr,$flags:expr) => {
        $crate::sys::zend_function_entry {
            fname: $fname,
            handler: $handler,
            arg_info: $arg_info,
            num_args: $num_args,
            flags: $flags,
        }
    };
}

#[macro_export]
macro_rules! define_zend_module_entry {
    ($name:expr,$functions:expr,
     $module_startup_func:expr,$module_shutdown_func:expr,$request_startup_func:expr,$request_shutdown_func:expr,
     $info_func:expr) => {
        NotThreadSafe(&$crate::sys::zend_module_entry {
            size: size_of::<$crate::sys::zend_module_entry>() as c_ushort,
            zend_api: $crate::sys::ZEND_MODULE_API_NO as c_uint,
            zend_debug: $crate::sys::ZEND_DEBUG as c_uchar,
            // TODO Don't support `ZTS` now.
            zts: 0 as c_uchar, // ::phper::sys::USING_ZTS as c_uchar,
            ini_entry: ::std::ptr::null(),
            deps: ::std::ptr::null(),
            name: $name,
            functions: $functions.0,
            module_startup_func: $module_startup_func,
            module_shutdown_func: $module_shutdown_func,
            request_startup_func: $request_startup_func,
            request_shutdown_func: $request_shutdown_func,
            info_func: $info_func,
            version: c_str_ptr!(env!("CARGO_PKG_VERSION")),
            globals_size: 0usize,
            globals_ptr: ::std::ptr::null_mut(),
            globals_ctor: None,
            globals_dtor: None,
            post_deactivate_func: None,
            module_started: 0,
            type_: 0,
            handle: ::std::ptr::null_mut(),
            module_number: 0,
            build_id: $crate::sys::PHP_BUILD_ID,
        } as *const $crate::sys::zend_module_entry)
    };
}

#[macro_export]
macro_rules! zstr_is_interned {
    ($s: expr) => {
        $crate::gc_flags!($s) as u32 & $crate::sys::IS_STR_INTERNED != 0
    };
}

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
        let __b: bool = $b;
        $crate::z_type_info!($z) = if __b {
            $crate::IS_TRUE
        } else {
            $crate::IS_FALSE
        };
    };
}

#[macro_export]
macro_rules! zval_long {
    ($z: expr, $l: expr) => {
        let __z: *mut $crate::zval = $z;
        $crate::z_lval_p(__z) = $l;
        $crate::z_type_info!(__z) = $crate::IS_LONG;
    };
}

#[macro_export]
macro_rules! zval_double {
    ($z: expr, $d: expr) => {
        let __z: *mut $crate::zval = $z;
        $crate::z_lval_p(__z) = $l;
        $crate::z_type_info!(__z) = $crate::IS_DOUBLE;
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
        let __z: *mut $crate::sys::zval = $z;
        let __s: *mut $crate::sys::zend_string = $s;
        $crate::z_str_p!(__z) = __s;
        $crate::z_type_info_p!(__z) = if $crate::zstr_is_interned!(__s) {
            $crate::sys::IS_INTERNED_STRING_EX
         } else {
			$crate::sys::IS_STRING_EX
         } as u32;
    };
}
