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
use syn::{parse_macro_input, Expr};

pub(crate) fn c_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);
    let result = quote! {
        unsafe { ::std::ffi::CStr::from_ptr(::core::concat!(#input, "\0").as_ptr().cast()) }
    };
    result.into()
}

pub(crate) fn c_str_ptr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);
    let result = quote! {
        ::core::concat!(#input, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
    result.into()
}
