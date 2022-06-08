// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{modules::Module, strings::ZendString, values::Val};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_strings_zend_string_new",
        |_: &mut [Val]| -> phper::Result<()> {
            let zs = ZendString::new("hello");
            assert_eq!(zs.as_str()?, "hello");

            let zs = ZendString::new([1, 2, 3]);
            assert_eq!(zs.as_ref(), &[1, 2, 3]);

            assert!(*ZendString::new("hello") == *ZendString::new(b"hello"));

            Ok(())
        },
        vec![],
    );
}
