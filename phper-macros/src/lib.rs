mod alloc;
mod inner;
mod utils;

use crate::inner::{hook_fn, info_fn};
use proc_macro::TokenStream;

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

#[proc_macro_attribute]
pub fn php_function(attr: TokenStream, input: TokenStream) -> TokenStream {
    inner::php_function(attr, input)
}

#[proc_macro_attribute]
pub fn php_get_module(attr: TokenStream, input: TokenStream) -> TokenStream {
    inner::php_get_module(attr, input)
}

#[proc_macro_attribute]
pub fn php_minit_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    hook_fn(input)
}

#[proc_macro_attribute]
pub fn php_mshutdown_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    hook_fn(input)
}

#[proc_macro_attribute]
pub fn php_rinit_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    hook_fn(input)
}

#[proc_macro_attribute]
pub fn php_rshutdown_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    hook_fn(input)
}

#[proc_macro_attribute]
pub fn php_ginit_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    hook_fn(input)
}

#[proc_macro_attribute]
pub fn php_gshutdown_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    hook_fn(input)
}

#[proc_macro_attribute]
pub fn php_minfo_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    info_fn(input)
}
