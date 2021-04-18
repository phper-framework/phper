mod alloc;
mod inner;
mod utils;

use proc_macro::TokenStream;

#[proc_macro]
pub fn c_str(input: TokenStream) -> TokenStream {
    utils::c_str(input)
}

#[proc_macro]
pub fn c_str_ptr(input: TokenStream) -> TokenStream {
    utils::c_str_ptr(input)
}

#[proc_macro_attribute]
pub fn php_get_module(attr: TokenStream, input: TokenStream) -> TokenStream {
    inner::php_get_module(attr, input)
}
