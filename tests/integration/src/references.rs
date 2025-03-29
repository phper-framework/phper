// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{functions::Argument, modules::Module};

#[allow(clippy::disallowed_names)]
pub fn integrate(module: &mut Module) {
    module
        .add_function("integrate_test_reference", |arguments| {
            let foo = arguments[0].expect_mut_z_ref()?;
            *foo.val_mut().expect_mut_long()? += 100;

            let bar = arguments[1].expect_z_ref()?;
            bar.val().expect_null()?;

            *arguments[1].as_mut_z_ref().unwrap().val_mut() = "hello".into();

            Ok::<_, phper::Error>(())
        })
        .arguments([Argument::new("foo").by_ref(), Argument::new("bar").by_ref()]);
}
