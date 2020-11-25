use crate::sys::{E_ERROR, E_NOTICE, E_WARNING};

#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
pub enum Level {
    Error = E_ERROR,
    Warning = E_WARNING,
    Notice = E_NOTICE,
}
