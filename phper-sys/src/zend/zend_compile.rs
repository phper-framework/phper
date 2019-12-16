#[macro_export]
macro_rules! zend_call_num_args {
    ($call: expr) => {
        (*$call).This.u2.num_args
    };
}
