// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper_macros::*;
use std::ffi::CStr;

#[test]
fn test_c_str() {
    assert_eq!(c_str!("foo"), unsafe {
        CStr::from_ptr("foo\0".as_ptr().cast())
    });

    assert_eq!(
        {
            #[allow(unused_unsafe)]
            unsafe {
                c_str!("bar")
            }
        },
        unsafe { CStr::from_ptr("bar\0".as_ptr().cast()) }
    );
}

#[test]
fn test_c_str_ptr() {
    assert_eq!(c_str_ptr!("foo"), "foo\0".as_ptr().cast());
}
