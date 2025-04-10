// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{modules::Module, strings::ZString, values::ZVal};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_strings_zend_string_new",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let zs = ZString::new("hello");
            assert_eq!(zs.to_str()?, "hello");

            let zs = ZString::new([1, 2, 3]);
            assert_eq!(zs.as_ref(), &[1, 2, 3]);

            assert!(ZString::new("hello") == ZString::new(b"hello"));

            Ok(())
        },
    );

    module.add_function(
        "integrate_strings_zend_string_new_persistent",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let zs = ZString::new_persistent("persistent_hello");
            assert_eq!(zs.to_str()?, "persistent_hello");

            let zs = ZString::new_persistent([4, 5, 6]);
            assert_eq!(zs.as_ref(), &[4, 5, 6]);

            assert!(
                ZString::new_persistent("persistent") == ZString::new_persistent(b"persistent")
            );

            Ok(())
        },
    );
}
