#![feature(min_const_generics)]

/*!
A library that allows us to write PHP extensions using pure Rust and using safe Rust whenever possible.

***Now the peojct is still under development.***

# Usage

First you have to install `cargo-generate`:

```bash
cargo install cargo-generate
```

Then create a PHP extension project from the [template](https://github.com/jmjoy/phper-ext-skel.git):

```bash
cargo generate --git https://github.com/jmjoy/phper-ext-skel.git
```

# Notice

Now the library don't support `ZTS`, the template is using `thread_local!` instead.

Version `0.1.x` will be a preview version.
*/

pub mod zend;
mod error;

pub use phper_alloc as alloc;
pub use phper_sys as sys;
pub use phper_macros::*;
pub use crate::error::*;

// pub extern crate phper_alloc as alloc;
// extern crate phper_macros;
// pub extern crate phper_sys as sys;
//
// mod macros;
//
// pub use phper_macros::*;
//
// mod arg;
// mod function;
// mod module;
// mod types;
//
// pub use arg::*;
// pub use function::*;
// pub use module::*;
// pub use types::*;
//
// use thiserror::Error;
//
// //pub type IniEntries = Vec<zend_ini_entry_def>;
// //
// //pub type StaticZendModuleEntry = NotThreadSafe<*const zend_module_entry>;
// //
// //pub type StaticZendFunctionEntry = NotThreadSafe<*const zend_function_entry>;
// //
// //pub struct NotThreadSafe<T>(pub T);
// //
// //unsafe impl<T> Sync for NotThreadSafe<T> {}
//
// #[doc(hidden)]
// pub fn wrap_php_function(
//     execute_data: *mut sys::zend_execute_data,
//     return_value: *mut sys::zval,
//     func: FunctionType,
// ) {
//     let parameters = Parameters { execute_data };
//     match func(parameters) {
//         Ok(_) => {}
//         Err(e) => panic!(e),
//     }
// }
