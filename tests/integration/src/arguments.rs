// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{
    alloc::ToRefOwned, arrays::ZArray, functions::Argument, modules::Module, objects::ZObject,
    values::ZVal,
};

pub fn integrate(module: &mut Module) {
    integrate_arguments(module);
}

fn integrate_arguments(module: &mut Module) {
    module.add_function(
        "integrate_arguments_null",
        |arguments: &mut [ZVal]| arguments[0].expect_null(),
        vec![Argument::by_val("a")],
    );

    module.add_function(
        "integrate_arguments_long",
        |arguments: &mut [ZVal]| -> phper::Result<i64> {
            let a = arguments[0].expect_long()?;
            arguments[1].convert_to_long();
            let b = arguments[1].as_long().unwrap();
            Ok(a + b)
        },
        vec![Argument::by_val("a"), Argument::by_val("b")],
    );

    module.add_function(
        "integrate_arguments_double",
        |arguments: &mut [ZVal]| arguments[0].expect_double(),
        vec![Argument::by_val("a")],
    );

    module.add_function(
        "integrate_arguments_string",
        |arguments: &mut [ZVal]| -> phper::Result<String> {
            let a = arguments[0].expect_z_str()?.to_str()?.to_owned();
            arguments[1].convert_to_string();
            let b = arguments[1].as_z_str().unwrap().to_str()?;
            Ok(format!("{}, {}", a, b))
        },
        vec![Argument::by_val("a"), Argument::by_val("b")],
    );

    module.add_function(
        "integrate_arguments_array",
        |arguments: &mut [ZVal]| -> phper::Result<ZArray> {
            let a = arguments[0].expect_z_arr()?;
            let mut b = a.to_owned();
            b.insert("a", ZVal::from(1));
            b.insert("foo", ZVal::from("bar"));
            Ok(b)
        },
        vec![Argument::by_val("a")],
    );

    module.add_function(
        "integrate_arguments_object",
        |arguments: &mut [ZVal]| -> phper::Result<ZObject> {
            let a = arguments[0].expect_mut_z_obj()?;
            let mut a = a.to_ref_owned();
            a.set_property("foo", ZVal::from("bar"));
            Ok(a)
        },
        vec![Argument::by_val("a")],
    );

    module.add_function(
        "integrate_arguments_optional",
        |arguments: &mut [ZVal]| -> phper::Result<String> {
            let a = arguments[0].expect_z_str()?.to_str()?;
            let b = arguments
                .get(1)
                .map(|b| b.expect_bool())
                .transpose()?
                .unwrap_or_default();
            Ok(format!("{}: {}", a, b))
        },
        vec![Argument::by_val("a"), Argument::by_val_optional("b")],
    );
}
