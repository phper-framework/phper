/// Wrapper of `EBox::new`.
///
/// # Examples
///
/// ```no_test
/// let _ = ebox!(1);
/// ```
#[macro_export]
macro_rules! ebox {
    ($arg:tt) => {{
        $crate::EBox::new($arg)
    }};
}
