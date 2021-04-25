#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        phper::logs::log(phper::logs::Level::Error, std::format!($($arg)*))
    })
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => ({
        phper::logs::log(phper::logs::Level::Warning, std::format!($($arg)*))
    })
}

#[macro_export]
macro_rules! notice {
    ($($arg:tt)*) => ({
        phper::logs::log(phper::logs::Level::Notice, std::format!($($arg)*))
    })
}

#[macro_export]
macro_rules! deprecated {
    ($($arg:tt)*) => ({
        phper::logs::log(phper::logs::Level::Deprecated, std::format!($($arg)*))
    })
}
