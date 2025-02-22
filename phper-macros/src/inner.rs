// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Visibility, parse_macro_input};

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
        #[unsafe(no_mangle)]
        #[doc(hidden)]
        #(#attrs)*
        #vis extern "C" fn #name() -> *const ::phper::sys::zend_module_entry {
            fn internal(#inputs) #ret {
                #body
            }
            let internal: fn() -> ::phper::modules::Module = internal;
            unsafe { internal().module_entry() }
        }
    };

    result.into()
}
