// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_constant].

use std::ffi::{c_char, c_int};

use crate::{modules::ModuleContext, sys::*, types::Scalar};

pub(crate) struct Constant {
    name: String,
    value: Scalar,
}

impl Constant {
    pub fn new(name: impl Into<String>, value: impl Into<Scalar>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    pub(crate) fn register(&self, module_context: &ModuleContext) {
        let name_ptr = self.name.as_ptr() as *const c_char;
        let name_len = self.name.len() as size_t;
        let flags = (CONST_PERSISTENT | CONST_CS) as c_int;
        let num = module_context.module_number;

        unsafe {
            match &self.value {
                Scalar::Null => zend_register_null_constant(name_ptr, name_len, flags, num),
                Scalar::Bool(b) => {
                    zend_register_bool_constant(name_ptr, name_len, *b as zend_bool, flags, num)
                }
                Scalar::I64(i) => {
                    zend_register_long_constant(name_ptr, name_len, *i as zend_long, flags, num)
                }
                Scalar::F64(f) => zend_register_double_constant(name_ptr, name_len, *f, flags, num),
                Scalar::String(s) => {
                    let s_ptr = s.as_ptr() as *mut u8;
                    zend_register_stringl_constant(
                        name_ptr,
                        name_len,
                        s_ptr.cast(),
                        s.len() as size_t,
                        flags,
                        num,
                    )
                }
                Scalar::Bytes(s) => {
                    let s_ptr = s.as_ptr() as *mut u8;
                    zend_register_stringl_constant(
                        name_ptr,
                        name_len,
                        s_ptr.cast(),
                        s.len() as size_t,
                        flags,
                        num,
                    )
                }
            }
        }
    }
}
