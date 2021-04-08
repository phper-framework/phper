use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Visibility};

pub(crate) fn hook_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let ret = &input.sig.output;
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #(#attrs)*
        extern "C" fn #name(type_: ::std::os::raw::c_int, module_number: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
            fn internal(#inputs) #ret {
                #body
            }

            let internal: fn(::phper::zend::modules::ModuleArgs) -> bool = internal;

            if internal(::phper::zend::modules::ModuleArgs::new(type_, module_number)) {
                ::phper::sys::ZEND_RESULT_CODE_SUCCESS
            } else {
                ::phper::sys::ZEND_RESULT_CODE_FAILURE
            }
        }
    };

    result.into()
}

pub(crate) fn info_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let ret = &input.sig.output;
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #(#attrs)*
        extern "C" fn #name(zend_module: *mut ::phper::sys::zend_module_entry) {
            fn internal(#inputs) #ret {
                #body
            }

            let internal: fn(&::phper::zend::modules::ModuleEntry) = internal;
            internal(::phper::zend::modules::ModuleEntry::from_ptr(zend_module))
        }
    };

    result.into()
}

pub(crate) fn php_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let vis = &input.vis;
    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let result = quote! {
        #(#attrs)*
        #vis extern "C" fn #name(
            execute_data: *mut ::phper::sys::zend_execute_data,
            return_value: *mut ::phper::sys::zval
        ) {
            fn internal(#inputs) #ret {
                #body
            }
            let internal: fn(&mut ::phper::zend::types::ExecuteData) -> _ = internal;
            unsafe {
                let value = internal(::phper::zend::types::ExecuteData::from_mut(execute_data));
                ::phper::zend::types::SetVal::set_val(value, ::phper::zend::types::Val::from_mut(return_value));
            }
        }
    };

    result.into()
}

pub(crate) fn php_get_module(_attr: TokenStream, input: TokenStream) -> TokenStream {
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
            let internal: fn() -> &::phper::zend::modules::ModuleEntry = internal;
            internal().as_ptr()
        }
    };

    result.into()
}
