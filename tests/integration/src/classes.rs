// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{
    alloc::RefClone,
    classes::{array_access_class, iterator_class, ClassEntity, Visibility},
    functions::Argument,
    modules::Module,
    values::ZVal,
};
use std::collections::HashMap;

pub fn integrate(module: &mut Module) {
    integrate_a(module);
    integrate_foo(module);
}

fn integrate_a(module: &mut Module) {
    let mut class = ClassEntity::new("IntegrationTest\\A");

    class.add_property("name", Visibility::Private, "default");
    class.add_property("number", Visibility::Private, 100);

    class
        .add_method("__construct", Visibility::Public, |this, arguments| {
            let name = arguments[0].expect_z_str()?.to_str()?;
            let number = arguments[1].expect_long()?;
            this.set_property("name", ZVal::from(name));
            this.set_property("number", ZVal::from(number));
            Ok::<_, phper::Error>(())
        })
        .arguments([Argument::by_val("name"), Argument::by_val("number")]);

    class.add_method("speak", Visibility::Public, |this, _arguments| {
        let name = this
            .get_property("name")
            .expect_z_str()?
            .to_str()?
            .to_owned();
        let number = this.get_property("number").expect_long()?;

        Ok::<_, phper::Error>(format!("name: {}, number: {}", name, number))
    });

    module.add_class(class);
}

struct Foo {
    position: usize,
    array: HashMap<i64, ZVal>,
}

fn integrate_foo(module: &mut Module) {
    let mut class = ClassEntity::new_with_state_constructor("IntegrationTest\\Foo", || Foo {
        position: 0,
        array: Default::default(),
    });

    class.implements(iterator_class);
    class.implements(array_access_class);

    // Implement Iterator interface.
    class.add_method("current", Visibility::Public, |this, _arguments| {
        let state = this.as_state();
        Ok::<_, phper::Error>(format!("Current: {}", state.position))
    });
    class.add_method("key", Visibility::Public, |this, _arguments| {
        let state = this.as_state();
        Ok::<_, phper::Error>(state.position as i64)
    });
    class.add_method("next", Visibility::Public, |this, _arguments| {
        let state = this.as_mut_state();
        state.position += 1;
    });
    class.add_method("rewind", Visibility::Public, |this, _arguments| {
        let state = this.as_mut_state();
        state.position = 0;
    });
    class.add_method("valid", Visibility::Public, |this, _arguments| {
        let state = this.as_state();
        state.position < 3
    });

    // Implement ArrayAccess interface.
    class
        .add_method("offsetExists", Visibility::Public, |this, arguments| {
            let offset = arguments[0].expect_long()?;
            let state = this.as_state();
            Ok::<_, phper::Error>(state.array.get(&offset).is_some())
        })
        .argument(Argument::by_val("offset"));

    class
        .add_method("offsetGet", Visibility::Public, |this, arguments| {
            let offset = arguments[0].expect_long()?;
            let state = this.as_mut_state();
            let val = state.array.get_mut(&offset).map(|val| val.ref_clone());
            Ok::<_, phper::Error>(val)
        })
        .argument(Argument::by_val("offset"));

    class
        .add_method("offsetSet", Visibility::Public, |this, arguments| {
            let offset = arguments[0].expect_long()?;
            let value = arguments[1].clone();
            let state = this.as_mut_state();
            state.array.insert(offset, value);
            Ok::<_, phper::Error>(())
        })
        .arguments([Argument::by_val("offset"), Argument::by_val("value")]);

    class
        .add_method("offsetUnset", Visibility::Public, |this, arguments| {
            let offset = arguments[0].expect_long()?;
            let state = this.as_mut_state();
            state.array.remove(&offset);
            Ok::<_, phper::Error>(())
        })
        .argument(Argument::by_val("offset"));

    module.add_class(class);
}
