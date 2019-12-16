#[macro_export]
macro_rules! emalloc {
    ($size: expr) => {
        $crate::_emalloc($size)
    };
}

#[macro_export]
macro_rules! efree {
    ($ptr: expr) => {
        $crate::_efree($ptr)
    };
}
