#[macro_export]
macro_rules! zstr_is_interned {
    ($s: expr) => {
        $crate::gc_flags!($s) as u32 & ::phper_sys::IS_STR_INTERNED != 0
    };
}
