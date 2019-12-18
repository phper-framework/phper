extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, parse_str, FnArg, ItemFn, NestedMeta, AttributeArgs, Meta};

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

    let name = &zend_module_startup_n(input.sig.ident.clone());
    let inputs = &init_func_args(Punctuated::new());
    let inner_inputs = &input.sig.inputs;
    let ret = &input.sig.output;
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #[no_mangle]
        #(#attrs)*
        pub extern "C" fn #name(#inputs) -> ::std::os::raw::c_int {
            unsafe {
                ::phper_sys::zend_register_ini_entries(
                    INI_ENTRIES.with(|i| i.as_ptr() as *const ::phper_sys::zend_ini_entry_def),
                    module_number
                );
            }

            let f = |#inner_inputs| #ret {
                #body
            };
            let b: bool = f();
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
pub fn php_mshutdown_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &zend_module_shutdown_n(input.sig.ident.clone());
    let inputs = &shutdown_func_args(Punctuated::new());
    let inner_inputs = &input.sig.inputs;
    let ret = &input.sig.output;
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #[no_mangle]
        #(#attrs)*
        pub extern "C" fn #name(#inputs) -> ::std::os::raw::c_int {
            unsafe {
                ::phper_sys::zend_unregister_ini_entries(module_number);
            }

            let f = |#inner_inputs| #ret {
                #body
            };
            let b: bool = f();
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

    let name = &zend_module_activate_n(input.sig.ident.clone());
    let inputs = &init_func_args(Punctuated::new());
    let inner_inputs = &input.sig.inputs;
    let ret = &input.sig.output;
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #[no_mangle]
        pub extern "C" fn #name(#inputs) -> ::std::os::raw::c_int {
            #(#attrs)*
            fn #name(#inner_inputs) #ret {
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
pub fn php_rshutdown_function(_attr: TokenStream, mut input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &zend_module_deactivate_n(input.sig.ident.clone());
    let inputs = &init_func_args(Punctuated::new());
    let inner_inputs = &input.sig.inputs;
    let ret = &input.sig.output;
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #[no_mangle]
        pub extern "C" fn #name(#inputs) -> ::std::os::raw::c_int {
            #(#attrs)*
            fn #name(#inner_inputs) #ret {
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
pub fn php_minfo_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &zend_module_info_n(input.sig.ident.clone());
    let inputs = &zend_module_info_func_args(Punctuated::new());
    let inner_inputs = &input.sig.inputs;
    let ret = &input.sig.output;
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #[no_mangle]
        #(#attrs)*
        pub extern "C" fn #name(#inputs) {
            let f = |#inner_inputs| #ret {
                #body
            };
            let _: () = f();
        }
    };

    result.into()
}

//#[proc_macro_attribute]
//pub fn php_ini(_attr: TokenStream, input: TokenStream) -> TokenStream {
//    input
//}

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

fn zend_module_info_func_args(mut inputs: Punctuated<FnArg, Comma>) -> Punctuated<FnArg, Comma> {
    inputs.push(parse_str("zend_module: *mut ::phper_sys::zend_module_entry").unwrap());
    inputs
}

fn zend_module_startup_n(ident: Ident) -> Ident {
    Ident::new(&format!("zm_startup_{}", ident), ident.span())
}

fn zend_module_shutdown_n(ident: Ident) -> Ident {
    Ident::new(&format!("zm_shutdown_{}", ident), ident.span())
}

fn zend_module_activate_n(ident: Ident) -> Ident {
    Ident::new(&format!("zm_activate_{}", ident), ident.span())
}

fn zend_module_deactivate_n(ident: Ident) -> Ident {
    Ident::new(&format!("zm_deactivate_{}", ident), ident.span())
}

fn zend_module_info_n(ident: Ident) -> Ident {
    Ident::new(&format!("zm_info_{}", ident), ident.span())
}
