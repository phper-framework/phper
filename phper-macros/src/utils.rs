use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Expr};

pub(crate) fn c_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);
    let result = quote! {
        unsafe { ::std::ffi::CStr::from_ptr(::core::concat!(#input, "\0").as_ptr().cast()) }
    };
    result.into()
}

pub(crate) fn c_str_ptr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);
    let result = quote! {
        ::core::concat!(#input, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
    result.into()
}
