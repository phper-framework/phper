#[macro_export]
macro_rules! emalloc {
    ( $( $x:expr ),* ) => {
        crate::_emalloc( $( $x, )* )
    };
}

#[macro_export]
macro_rules! efree {
    ( $( $x:expr ),* ) => {
        crate::_efree( $( $x, )* )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emalloc_efree() {
        unsafe {
            let ptr = emalloc!(1);
            efree!(ptr);
        }
    }
}