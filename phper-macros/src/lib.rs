extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, parse_str, FnArg, Ident, ItemFn};

#[proc_macro_attribute]
pub fn php_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let vis = &input.vis;
    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    //    let mut inputs = &mut inputs.clone();
    //    internal_function_parameters(&mut inputs);

    let result = quote! {
        #(#attrs)*
        #vis extern "C" fn #name(
            execute_data: *mut ::phper::sys::zend_execute_data,
            return_value: *mut ::phper::sys::zval
        ) {
            fn internal(#inputs) #ret {
                #body
            }
            let internal: ::phper::FunctionType = internal;
            ::phper::wrap_php_function(execute_data, return_value, internal);
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn php_get_module(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let vis = &input.vis;
    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    if name != "get_module" {
        panic!("function name of `php_get_module` must be `get_module`");
    }

    let result = quote! {
        #[no_mangle]
        #(#attrs)*
        #vis extern "C" fn #name() -> *const ::phper::sys::zend_module_entry {
            fn internal(#inputs) #ret {
                #body
            }
            let internal: fn() -> ::phper::Result<::phper::Module<'static>> = internal;
            let module: ::phper::Module = internal().expect("Get module failed");
            module.into()
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn php_minit_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &input.sig.ident;
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
                ::phper::sys::zend_register_ini_entries(
                    INI_ENTRIES.with(|i| i.as_ptr() as *const ::phper::sys::zend_ini_entry_def),
                    module_number
                );
            }

            let f = |#inner_inputs| #ret {
                #body
            };
            let b: bool = f();
            if b {
                ::phper::sys::ZEND_RESULT_CODE_SUCCESS
            } else {
                ::phper::sys::ZEND_RESULT_CODE_FAILURE
            }
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn php_mshutdown_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &input.sig.ident;
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
                ::phper::sys::zend_unregister_ini_entries(module_number);
            }

            let f = |#inner_inputs| #ret {
                #body
            };
            let b: bool = f();
            if b {
                ::phper::sys::ZEND_RESULT_CODE_SUCCESS
            } else {
                ::phper::sys::ZEND_RESULT_CODE_FAILURE
            }
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn php_rinit_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &input.sig.ident;
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
                ::phper::sys::ZEND_RESULT_CODE_SUCCESS
            } else {
                ::phper::sys::ZEND_RESULT_CODE_FAILURE
            }
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn php_rshutdown_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &input.sig.ident;
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
                ::phper::sys::ZEND_RESULT_CODE_SUCCESS
            } else {
                ::phper::sys::ZEND_RESULT_CODE_FAILURE
            }
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn php_minfo_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &input.sig.ident;
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

#[proc_macro]
pub fn zend_get_module(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);

    let result = quote! {
        #[no_mangle]
        pub extern "C" fn get_module() -> *const ::phper::sys::zend_module_entry {
            #name.0
        }
    };

    result.into()
}

fn internal_function_parameters(inputs: &mut Punctuated<FnArg, Comma>) {
    inputs.push(parse_str("execute_data: *mut ::phper::sys::zend_execute_data").unwrap());
    inputs.push(parse_str("return_value: *mut ::phper::sys::zval").unwrap());
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
    inputs.push(parse_str("zend_module: *mut ::phper::sys::zend_module_entry").unwrap());
    inputs
}

//fn zend_module_startup_n(ident: Ident) -> Ident {
//    Ident::new(&format!("zm_startup_{}", ident), ident.span())
//}
//
//fn zend_module_shutdown_n(ident: Ident) -> Ident {
//    Ident::new(&format!("zm_shutdown_{}", ident), ident.span())
//}
//
//fn zend_module_activate_n(ident: Ident) -> Ident {
//    Ident::new(&format!("zm_activate_{}", ident), ident.span())
//}
//
//fn zend_module_deactivate_n(ident: Ident) -> Ident {
//    Ident::new(&format!("zm_deactivate_{}", ident), ident.span())
//}
//
//fn zend_module_info_n(ident: Ident) -> Ident {
//    Ident::new(&format!("zm_info_{}", ident), ident.span())
//}
