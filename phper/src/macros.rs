#[macro_export]
macro_rules! echo {
    ($($arg:tt)*) => ({
        $crate::output::echo(std::format!($($arg)*))
    })
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::output::log($crate::output::LogLevel::Error, std::format!($($arg)*))
    })
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => ({
        $crate::output::log($crate::output::LogLevel::Warning, std::format!($($arg)*))
    })
}

#[macro_export]
macro_rules! notice {
    ($($arg:tt)*) => ({
        $crate::output::log($crate::output::LogLevel::Notice, std::format!($($arg)*))
    })
}

#[macro_export]
macro_rules! deprecated {
    ($($arg:tt)*) => ({
        $crate::output::log($crate::output::LogLevel::Deprecated, std::format!($($arg)*))
    })
}
