mod alloc;
mod inner;
mod utils;

use proc_macro::TokenStream;
use crate::inner::{rename, hook_fn, info_fn};

#[proc_macro]
pub fn c_str(input: TokenStream) -> TokenStream {
    utils::c_str(input)
}

#[proc_macro]
pub fn c_str_ptr(input: TokenStream) -> TokenStream {
    utils::c_str_ptr(input)
}

#[proc_macro]
pub fn ebox(input: TokenStream) -> TokenStream {
    alloc::ebox(input)
}

#[proc_macro]
pub fn php_fn(input: TokenStream) -> TokenStream {
    rename(input, "zif_")
}

#[proc_macro]
pub fn php_mn(input: TokenStream) -> TokenStream {
    rename(input, "zim_")
}

#[proc_macro]
pub fn php_minit(input: TokenStream) -> TokenStream {
    rename(input, "zm_startup_")
}

#[proc_macro]
pub fn php_mshutdown(input: TokenStream) -> TokenStream {
    rename(input, "zm_shutdown_")
}

#[proc_macro]
pub fn php_rinit(input: TokenStream) -> TokenStream {
    rename(input, "zm_activate_")
}

#[proc_macro]
pub fn php_rshutdown(input: TokenStream) -> TokenStream {
    rename(input, "zm_deactivate_")
}

#[proc_macro]
pub fn php_minfo(input: TokenStream) -> TokenStream {
    rename(input, "zm_info_")
}

#[proc_macro]
pub fn php_ginfo(input: TokenStream) -> TokenStream {
    rename(input, "zm_globals_ctor_")
}

#[proc_macro]
pub fn php_gshutdown(input: TokenStream) -> TokenStream {
    rename(input, "zm_globals_dtor_")
}

#[proc_macro_attribute]
pub fn php_function(attr: TokenStream, input: TokenStream) -> TokenStream {
    inner::php_function(attr, input)
}

#[proc_macro_attribute]
pub fn zend_get_module(attr: TokenStream, input: TokenStream) -> TokenStream {
    inner::zend_get_module(attr, input)
}

#[proc_macro_attribute]
pub fn php_minit_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    hook_fn(input, "zm_startup_")
}

#[proc_macro_attribute]
pub fn php_mshutdown_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    hook_fn(input, "zm_shutdown_")
}

#[proc_macro_attribute]
pub fn php_rinit_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    hook_fn(input, "zm_activate_")
}

#[proc_macro_attribute]
pub fn php_rshutdown_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    hook_fn(input, "zm_deactivate_")
}

#[proc_macro_attribute]
pub fn php_minfo_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    info_fn(input, "zm_info_")
}
