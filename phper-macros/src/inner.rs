use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Visibility};

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
            let internal: fn(module: &mut ::phper::modules::Module) = internal;

            ::phper::modules::write_global_module(internal);
            unsafe {
                ::phper::modules::read_global_module(|module| {
                    module.module_entry()
                })
            }
        }
    };

    result.into()
}
