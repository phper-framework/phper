#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]

/*!
The proc-macros for [phper](https://crates.io/crates/phper).

## License

[Unlicense](https://github.com/jmjoy/phper/blob/master/LICENSE).
*/

// TODO Write a bridge macro for easy usage about register functions and classes, like `cxx`.

mod alloc;
mod inner;
mod log;
mod utils;

use proc_macro::TokenStream;

/// C style string end with '\0'.
///
/// # Examples
///
/// ```no_test
/// use std::ffi::CStr;
///
/// assert_eq!(c_str!("foo"), unsafe {
///     CStr::from_ptr("foo\0".as_ptr().cast())
/// });
/// ```
#[proc_macro]
pub fn c_str(input: TokenStream) -> TokenStream {
    utils::c_str(input)
}

/// C style string end with '\0'.
///
/// # Examples
///
/// ```no_test
/// assert_eq!(c_str_ptr!("foo"), "foo\0".as_ptr().cast());
/// ```
#[proc_macro]
pub fn c_str_ptr(input: TokenStream) -> TokenStream {
    utils::c_str_ptr(input)
}

/// PHP module entry, wrap the `phper::modules::Module` write operation.
///
/// # Examples
///
/// ```no_test
/// use phper::{php_get_module, modules::Module};
///
/// #[php_get_module]
/// pub fn get_module() -> Module {
///     let mut module = Module::new(
///         env!("CARGO_PKG_NAME"),
///         env!("CARGO_PKG_VERSION"),
///         env!("CARGO_PKG_AUTHORS"),
///     );
///
///     // ...
///
///     module
/// }
///
/// ```
#[proc_macro_attribute]
pub fn php_get_module(attr: TokenStream, input: TokenStream) -> TokenStream {
    inner::php_get_module(attr, input)
}
