// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_constant](crate::sys::zend_constant).

use crate::{sys::*, types::Scalar};
use std::ffi::{c_char, c_int};

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

    pub(crate) fn register(&self, module_number: c_int) {
        let name_ptr = self.name.as_ptr() as *const c_char;
        let name_len = self.name.len();
        let flags = (CONST_PERSISTENT | CONST_CS) as c_int;

        unsafe {
            match &self.value {
                Scalar::Null => {
                    zend_register_null_constant(name_ptr, name_len, flags, module_number)
                }
                Scalar::Bool(b) => zend_register_bool_constant(
                    name_ptr,
                    name_len,
                    *b as zend_bool,
                    flags,
                    module_number,
                ),
                Scalar::I64(i) => zend_register_long_constant(
                    name_ptr,
                    name_len,
                    *i as zend_long,
                    flags,
                    module_number,
                ),
                Scalar::F64(f) => {
                    zend_register_double_constant(name_ptr, name_len, *f, flags, module_number)
                }
                Scalar::String(s) => {
                    let s_ptr = s.as_ptr() as *mut u8;
                    zend_register_stringl_constant(
                        name_ptr,
                        name_len,
                        s_ptr.cast(),
                        s.len(),
                        flags,
                        module_number,
                    )
                }
                Scalar::Bytes(s) => {
                    let s_ptr = s.as_ptr() as *mut u8;
                    zend_register_stringl_constant(
                        name_ptr,
                        name_len,
                        s_ptr.cast(),
                        s.len(),
                        flags,
                        module_number,
                    )
                }
            }
        }
    }
}
