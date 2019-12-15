extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, FnArg, parse_str, ExprLet};
use quote::quote;
use proc_macro2::{Ident, Span};
use syn::punctuated::Punctuated;
use syn::token::Comma;

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

    let vis = &input.vis;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let mut inputs = &mut inputs.clone();
    init_func_args(&mut inputs);

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

    let vis = &input.vis;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let mut inputs = &mut inputs.clone();
    shutdown_func_args(&mut inputs);

    let name = Ident::new(&format!("zm_shutdown_{}", name), Span::call_site());

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
pub fn php_rinit_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let vis = &input.vis;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let mut inputs = &mut inputs.clone();
    init_func_args(&mut inputs);

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
pub fn php_rshutdown_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let vis = &input.vis;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let mut inputs = &mut inputs.clone();
    shutdown_func_args(&mut inputs);

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

//#[proc_macro_attribute]
//pub fn zend_parse_parameters(_attr: TokenStream, input: TokenStream) -> TokenStream {
//    dbg!(&input);
//
////    let input = parse_macro_input!(input as ExprLet);
//
//    input
//}

#[proc_macro_attribute]
pub fn hehe(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

fn internal_function_parameters(inputs: &mut Punctuated<FnArg, Comma>) {
    inputs.push(parse_str("execute_data: *mut ::phper_sys::zend_execute_data").unwrap());
    inputs.push(parse_str("return_value: *mut ::phper_sys::zval").unwrap());
}

fn init_func_args(inputs: &mut Punctuated<FnArg, Comma>) {
    inputs.push(parse_str("r#type: ::std::os::raw::c_int").unwrap());
    inputs.push(parse_str("module_number: ::std::os::raw::c_int").unwrap());
}

fn shutdown_func_args(inputs: &mut Punctuated<FnArg, Comma>) {
    inputs.push(parse_str("r#type: ::std::os::raw::c_int").unwrap());
    inputs.push(parse_str("module_number: ::std::os::raw::c_int").unwrap());
}



