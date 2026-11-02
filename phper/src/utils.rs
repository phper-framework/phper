// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Internal useful utils.

use cfg_if::cfg_if;
use std::ffi::CString;

cfg_if! {
    if #[cfg(any(
        phper_major_version = "7",
        all(
            phper_major_version = "8",
            any(
                phper_minor_version = "0",
                phper_minor_version = "1",
                phper_minor_version = "2",
                phper_minor_version = "3",
                phper_minor_version = "4",
                phper_minor_version = "5",
            )
        )
    ))] {
        pub(crate) use crate::sys::{
            ZEND_EVAL_CODE, ZEND_RESULT_CODE_FAILURE, ZEND_RESULT_CODE_SUCCESS,
            ZEND_USER_FUNCTION,
        };
    } else {
        // PHP 8.6 made these C23 enums, so bindgen prefixes them with the
        // enum name. Alias them back so call sites read the same everywhere.
        use crate::sys::{zend_function_type_ZEND_EVAL_CODE, zend_function_type_ZEND_USER_FUNCTION, zend_result, zend_result_FAILURE, zend_result_SUCCESS};

        pub(crate) const ZEND_USER_FUNCTION: u32 = zend_function_type_ZEND_USER_FUNCTION as u32;
        pub(crate) const ZEND_EVAL_CODE: u32 = zend_function_type_ZEND_EVAL_CODE as u32;
        pub(crate) const ZEND_RESULT_CODE_SUCCESS: zend_result = zend_result_SUCCESS;
        pub(crate) const ZEND_RESULT_CODE_FAILURE: zend_result = zend_result_FAILURE;
    }
}

pub(crate) fn ensure_end_with_zero(s: impl Into<String>) -> CString {
    CString::new(s.into()).expect("CString::new failed")
}
