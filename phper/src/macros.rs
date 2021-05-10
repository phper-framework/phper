/// PHP echo.
///
/// # Examples
///
/// ```no_test
/// phper::echo!("Hello, {}!", message)
/// ```
#[macro_export]
macro_rules! echo {
    ($($arg:tt)*) => ({
        $crate::output::echo(std::format!($($arg)*))
    })
}

/// PHP error logging, will exit the request.
///
/// # Examples
///
/// ```no_test
/// phper::errro!("Hello, {}!", message)
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::output::log($crate::output::LogLevel::Error, std::format!($($arg)*))
    })
}

/// PHP warning logging.
///
/// # Examples
///
/// ```no_test
/// phper::warning!("Hello, {}!", message)
/// ```
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => ({
        $crate::output::log($crate::output::LogLevel::Warning, std::format!($($arg)*))
    })
}

/// PHP notice logging.
///
/// # Examples
///
/// ```no_test
/// phper::notice!("Hello, {}!", message)
/// ```
#[macro_export]
macro_rules! notice {
    ($($arg:tt)*) => ({
        $crate::output::log($crate::output::LogLevel::Notice, std::format!($($arg)*))
    })
}

/// PHP deprecated logging.
///
/// # Examples
///
/// ```no_test
/// phper::deprecated!("Hello, {}!", message)
/// ```
#[macro_export]
macro_rules! deprecated {
    ($($arg:tt)*) => ({
        $crate::output::log($crate::output::LogLevel::Deprecated, std::format!($($arg)*))
    })
}
