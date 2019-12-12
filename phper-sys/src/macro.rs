#[macro_export]
macro_rules! emalloc {
    ( $( $x:expr ),* ) => {
        $crate::_emalloc( $( $x, )* )
    };
}

#[macro_export]
macro_rules! efree {
    ( $( $x:expr ),* ) => {
        $crate::_efree( $( $x, )* )
    };
}
