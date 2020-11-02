use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn, Visibility};

pub(crate) fn rename(input: TokenStream, prefix: impl ToString) -> TokenStream {
    let input = parse_macro_input!(input as Ident);
    let name = prefix.to_string() + &input.to_string();
    let name = Ident::new(&name, input.span().clone());
    let result = quote! { #name };
    result.into()
}

pub(crate) fn hook_fn(input: TokenStream, prefix: impl ToString) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = prefix.to_string() + &input.sig.ident.to_string();
    let name = Ident::new(&name, input.sig.ident.span());
    let inputs = &input.sig.inputs;
    let ret = &input.sig.output;
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #[allow(dead_code)]
        #input

        #(#attrs)*
        extern "C" fn #name(type_: ::std::os::raw::c_int, module_number: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
            fn internal(#inputs) #ret {
                #body
            }

            let internal: fn(::std::os::raw::c_int, ::std::os::raw::c_int) -> bool = internal;

            if internal(type_, module_number) {
                ::phper::sys::ZEND_RESULT_CODE_SUCCESS
            } else {
                ::phper::sys::ZEND_RESULT_CODE_FAILURE
            }
        }
    };

    result.into()
}
pub(crate) fn info_fn(input: TokenStream, prefix: impl ToString) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = prefix.to_string() + &input.sig.ident.to_string();
    let name = Ident::new(&name, input.sig.ident.span());
    let inputs = &input.sig.inputs;
    let ret = &input.sig.output;
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #[allow(dead_code)]
        #input

        #(#attrs)*
        extern "C" fn #name(zend_module: *mut ::phper::sys::zend_module_entry) {
            fn internal(#inputs) #ret {
                #body
            }

            let internal: fn(*mut ::phper::sys::zend_module_entry) = internal;
            internal(zend_module)
        }
    };

    result.into()
}

pub(crate) fn php_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let vis = &input.vis;
    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = Ident::new(&format!("zif_{}", &input.sig.ident.to_string()), input.sig.ident.span().clone());
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #[allow(dead_code)]
        #input

        #(#attrs)*
        #vis extern "C" fn #name(
            execute_data: *mut ::phper::sys::zend_execute_data,
            return_value: *mut ::phper::sys::zval
        ) {
            fn internal(#inputs) #ret {
                #body
            }
            // let internal: fn(::phper::zend::types::ExecuteData) -> impl ::phper::zend::types::SetVal = internal;
            let value = internal(::phper::zend::types::ExecuteData::from_raw(execute_data));
            ::phper::zend::types::SetVal::set_val(value, &mut ::phper::zend::types::Val::from_raw(return_value));
        }
    };

    result.into()
}

pub(crate) fn zend_get_module(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let vis = &input.vis;
    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    if name != "get_module" {
        return quote! { compile_error!("function name with attribute `php_get_module` must be `get_module`") }.into();
    }

    if !matches!(vis, Visibility::Public(..)) {
        return quote! { compile_error!("function `get_module` must be public"); }.into();
    }

    let result = quote! {
        #[no_mangle]
        #(#attrs)*
        #vis extern "C" fn #name() -> *const ::phper::sys::zend_module_entry {
            fn internal(#inputs) #ret {
                #body
            }
            let internal: fn() -> &'static ::phper::zend::modules::ModuleEntry = internal;
            internal().as_ptr()
        }
    };

    result.into()
}
