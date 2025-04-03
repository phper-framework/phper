// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

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

/// Equivalent to the php `CG`.
#[macro_export]
macro_rules! cg {
    ($x:ident) => {
        $crate::sys::compiler_globals.$x
    };
}

/// Equivalent to the php `EG`.
#[macro_export]
macro_rules! eg {
    ($x:ident) => {
        $crate::sys::executor_globals.$x
    };
}

/// Equivalent to the php `PG`.
#[macro_export]
macro_rules! pg {
    ($x:ident) => {
        $crate::sys::core_globals.$x
    };
}

/// Equivalent to the php `SG`.
#[macro_export]
macro_rules! sg {
    ($x:ident) => {
        $crate::sys::sapi_globals.$x
    };
}

/// Define a class stub
#[macro_export]
macro_rules! define_class_stub {
    ($name:expr, $cls_var:ident) => {{
        use std::{cell::RefCell, rc::Rc};

        let $cls_var: Rc<RefCell<Option<_>>> = Rc::new(RefCell::new(None));
        let entity = ::phper::classes::ClassEntity::new($name);
        (entity, $cls_var)
    }};
}
