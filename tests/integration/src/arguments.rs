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
    alloc::EBox, arrays::ZArray, functions::Argument, modules::Module, objects::Object,
    values::ZVal,
};

pub fn integrate(module: &mut Module) {
    integrate_arguments(module);
}

fn integrate_arguments(module: &mut Module) {
    module.add_function(
        "integrate_arguments_null",
        |arguments: &mut [ZVal]| arguments[0].as_null(),
        vec![Argument::by_val("a")],
    );

    module.add_function(
        "integrate_arguments_long",
        |arguments: &mut [ZVal]| -> phper::Result<i64> {
            let a = arguments[0].as_long()?;
            let b = arguments[1].as_long_value();
            Ok(a + b)
        },
        vec![Argument::by_val("a"), Argument::by_val("b")],
    );

    module.add_function(
        "integrate_arguments_double",
        |arguments: &mut [ZVal]| arguments[0].as_double(),
        vec![Argument::by_val("a")],
    );

    module.add_function(
        "integrate_arguments_string",
        |arguments: &mut [ZVal]| -> phper::Result<String> {
            let a = arguments[0].to_string()?;
            let b = arguments[1].as_string_value()?;
            Ok(format!("{}, {}", a, b))
        },
        vec![Argument::by_val("a"), Argument::by_val("b")],
    );

    module.add_function(
        "integrate_arguments_array",
        |arguments: &mut [ZVal]| -> phper::Result<EBox<ZArray>> {
            let a = arguments[0].as_z_arr()?;
            let mut b = a.clone_arr();
            b.insert("a", ZVal::new(1));
            b.insert("foo", ZVal::new("bar"));
            Ok(b)
        },
        vec![Argument::by_val("a")],
    );

    module.add_function(
        "integrate_arguments_object",
        |arguments: &mut [ZVal]| -> phper::Result<EBox<Object<()>>> {
            let a = arguments[0].as_object()?;
            let mut a = a.clone_obj();
            a.set_property("foo", ZVal::new("bar"));
            Ok(a)
        },
        vec![Argument::by_val("a")],
    );

    module.add_function(
        "integrate_arguments_optional",
        |arguments: &mut [ZVal]| -> phper::Result<String> {
            let a = arguments[0].to_string()?;
            let b = arguments
                .get(1)
                .map(|b| b.as_bool())
                .transpose()?
                .unwrap_or_default();
            Ok(format!("{}: {}", a, b))
        },
        vec![Argument::by_val("a"), Argument::by_val_optional("b")],
    );
}
