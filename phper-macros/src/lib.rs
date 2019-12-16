extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, parse_str, FnArg, ItemFn};

#[proc_macro_attribute]
pub fn php_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let vis = &input.vis;
    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let mut inputs = &mut inputs.clone();
    internal_function_parameters(&mut inputs);

    let name = Ident::new(&format!("zif_{}", name), Span::call_site());

    let result = quote! {
        #[no_mangle]
        #(#attrs)*
        #vis extern "C" fn #name(#inputs) #ret {
            #body
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn php_minit_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    use syn::export::quote::ToTokens;

    let fn_arg = get_context_fn_arg(&input.sig.inputs);

    let vis = &input.vis;
    let inputs = &init_func_args(Punctuated::new());
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let name = Ident::new(&format!("zm_startup_{}", name), Span::call_site());

    let result = quote! {
        #[no_mangle]
        #(#attrs)*
        #vis extern "C" fn #name(#inputs) -> ::std::os::raw::c_int {
            #body
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn php_mshutdown_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &zend_module_shutdown_n(input.sig.ident.clone());
    let ret = &input.sig.output;
    let inputs = shutdown_func_args(Punctuated::new());
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #[no_mangle]
        pub extern "C" fn #name(#inputs) -> ::std::os::raw::c_int {
            #(#attrs)*
            fn #name() #ret {
                #body
            }
            let b: bool = #name();
            if b {
                ::phper_sys::ZEND_RESULT_CODE_SUCCESS
            } else {
                ::phper_sys::ZEND_RESULT_CODE_FAILURE
            }
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn php_rinit_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let vis = &input.vis;
    let inputs = &init_func_args(input.sig.inputs.clone());
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let name = Ident::new(&format!("zm_activate_{}", name), Span::call_site());

    let result = quote! {
        #[no_mangle]
        #(#attrs)*
        #vis extern "C" fn #name(#inputs) -> ::std::os::raw::c_int {
            #body
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn php_rshutdown_function(_attr: TokenStream, mut input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let vis = &input.vis;
    let inputs = &shutdown_func_args(input.sig.inputs.clone());
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let name = Ident::new(&format!("zm_deactivate_{}", name), Span::call_site());

    let result = quote! {
        #[no_mangle]
        #(#attrs)*
        #vis extern "C" fn #name(#inputs) -> ::std::os::raw::c_int {
            #body
        }
    };

    result.into()
}

fn internal_function_parameters(inputs: &mut Punctuated<FnArg, Comma>) {
    inputs.push(parse_str("execute_data: *mut ::phper_sys::zend_execute_data").unwrap());
    inputs.push(parse_str("return_value: *mut ::phper_sys::zval").unwrap());
}

fn init_func_args(mut inputs: Punctuated<FnArg, Comma>) -> Punctuated<FnArg, Comma> {
    inputs.push(parse_str("r#type: ::std::os::raw::c_int").unwrap());
    inputs.push(parse_str("module_number: ::std::os::raw::c_int").unwrap());
    inputs
}

fn shutdown_func_args(mut inputs: Punctuated<FnArg, Comma>) -> Punctuated<FnArg, Comma> {
    inputs.push(parse_str("r#type: ::std::os::raw::c_int").unwrap());
    inputs.push(parse_str("module_number: ::std::os::raw::c_int").unwrap());
    inputs
}

fn zend_module_info_func_args(inputs: &mut Punctuated<FnArg, Comma>) {
    inputs.push(parse_str("zend_module_entry: *mut ::phper_sys::zend_module").unwrap());
}

fn zend_module_shutdown_n(ident: Ident) -> Ident {
    Ident::new(&format!("zm_shutdown_{}", ident), ident.span())
}

fn get_context_fn_arg(inputs: &Punctuated<FnArg, Comma>) -> Option<&FnArg> {
    inputs.iter().find(|fn_arg| match fn_arg {
        FnArg::Typed(pat_type) => pat_type.attrs.iter().any(|attr| {
            attr.path
                .segments
                .iter()
                .any(|seg| seg.ident.to_string() == "context".to_string())
        }),
        _ => false,
    })
}
