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
    classes::{DynamicClass, Visibility},
    functions::Argument,
    modules::Module,
    values::Val,
};

pub fn integrate(module: &mut Module) {
    integrate_a(module);
}

fn integrate_a(module: &mut Module) {
    let mut class = DynamicClass::new("IntegrationTest\\A");

    class.add_property("name", Visibility::Private, "default");
    class.add_property("number", Visibility::Private, 100);

    class.add_method(
        "__construct",
        Visibility::Public,
        |this, arguments| {
            let name = arguments[0].as_string()?;
            let number = arguments[1].as_long()?;
            this.set_property("name", Val::new(name));
            this.set_property("number", Val::new(number));
            Ok::<_, phper::Error>(())
        },
        vec![Argument::by_val("name"), Argument::by_val("number")],
    );

    class.add_method(
        "speak",
        Visibility::Public,
        |this, _arguments| {
            let name = this.get_property("name").as_string()?;
            let number = this.get_property("number").as_long()?;

            Ok::<_, phper::Error>(format!("name: {}, number: {}", name, number))
        },
        vec![],
    );

    module.add_class(class);
}
