use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

pub fn eg(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Ident);
    (quote! {
        phper::sys::executor_globals.#input
    })
    .into()
}
