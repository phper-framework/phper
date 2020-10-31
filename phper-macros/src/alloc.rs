use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

pub(crate) fn ebox(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);
    let result = quote! {
        ::phper::alloc::EBox::new_in(#input, ::phper::alloc::Allocator::new(#[cfg(phper_debug)] ::phper::c_str_ptr!(file!()), #[cfg(phper_debug)] ::std::line!(), #[cfg(phper_debug)] ::std::ptr::null(), #[cfg(phper_debug)] 0))
    };
    result.into()
}
