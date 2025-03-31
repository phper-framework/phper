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
    classes::{
        ClassEntity, ClassEntry, Interface, InterfaceEntity, Visibility,
    },
    functions::{Argument, ReturnType},
    modules::Module,
    types::{ArgumentTypeHint, ReturnTypeHint},
    values::ZVal,
};
use std::{collections::HashMap, convert::Infallible};

pub fn integrate(module: &mut Module) {
    integrate_a(module);
    integrate_foo(module);
    integrate_i_bar(module);
    integrate_static_props(module);
    integrate_i_constants(module);
    integrate_bar_extends_foo(module);
    #[cfg(phper_major_version = "8")]
    integrate_stringable(module);
}

fn integrate_a(module: &mut Module) {
    let mut class = ClassEntity::new("IntegrationTest\\A");
    let integrate_a_class = class.bind_class();

    class.add_property("name", Visibility::Private, "default");
    class.add_property("number", Visibility::Private, 100);
    class.add_constant("CST_STRING", "foo");
    class.add_constant("CST_NULL", ());
    class.add_constant("CST_TRUE", true);
    class.add_constant("CST_FALSE", false);
    class.add_constant("CST_INT", 100);
    class.add_constant("CST_FLOAT", 10.0);

    class
        .add_method("__construct", Visibility::Public, |this, arguments| {
            let name = arguments[0].expect_z_str()?.to_str()?;
            let number = arguments[1].expect_long()?;
            this.set_property("name", ZVal::from(name));
            this.set_property("number", ZVal::from(number));
            Ok::<_, phper::Error>(())
        })
        .arguments([Argument::new("name"), Argument::new("number")]);

    class.add_static_method("newInstance", Visibility::Public, move |_| {
        let object = integrate_a_class.init_object()?;
        Ok::<_, phper::Error>(object)
    });

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

    class.implements(Interface::from_name("Iterator"));
    class.implements(Interface::from_name("ArrayAccess"));

    // Implement Iterator interface.
    class
        .add_method("current", Visibility::Public, |this, _arguments| {
            let state = this.as_state();
            Ok::<_, phper::Error>(format!("Current: {}", state.position))
        })
        .return_type(ReturnType::new(ReturnTypeHint::Mixed));

    class
        .add_method("key", Visibility::Public, |this, _arguments| {
            let state = this.as_state();
            Ok::<_, phper::Error>(state.position as i64)
        })
        .return_type(ReturnType::new(ReturnTypeHint::Mixed));

    class
        .add_method("next", Visibility::Public, |this, _arguments| {
            let state = this.as_mut_state();
            state.position += 1;
            Ok::<_, Infallible>(())
        })
        .return_type(ReturnType::new(ReturnTypeHint::Void));

    class
        .add_method("rewind", Visibility::Public, |this, _arguments| {
            let state = this.as_mut_state();
            state.position = 0;
            Ok::<_, Infallible>(())
        })
        .return_type(ReturnType::new(ReturnTypeHint::Void));

    class
        .add_method("valid", Visibility::Public, |this, _arguments| {
            let state = this.as_state();
            Ok::<_, Infallible>(state.position < 3)
        })
        .return_type(ReturnType::new(ReturnTypeHint::Bool));

    // Implement ArrayAccess interface.
    class
        .add_method("offsetExists", Visibility::Public, |this, arguments| {
            let offset = arguments[0].expect_long()?;
            let state = this.as_state();
            Ok::<_, phper::Error>(state.array.contains_key(&offset))
        })
        .argument(Argument::new("offset").with_type_hint(ArgumentTypeHint::Mixed))
        .return_type(ReturnType::new(ReturnTypeHint::Bool));

    class
        .add_method("offsetGet", Visibility::Public, |this, arguments| {
            let offset = arguments[0].expect_long()?;
            let state = this.as_mut_state();
            let val = state.array.get_mut(&offset).map(|val| val.ref_clone());
            Ok::<_, phper::Error>(val)
        })
        .argument(Argument::new("offset").with_type_hint(ArgumentTypeHint::Mixed))
        .return_type(ReturnType::new(ReturnTypeHint::Mixed));

    class
        .add_method("offsetSet", Visibility::Public, |this, arguments| {
            let offset = arguments[0].expect_long()?;
            let value = arguments[1].clone();
            let state = this.as_mut_state();
            state.array.insert(offset, value);
            Ok::<_, phper::Error>(())
        })
        .arguments([
            Argument::new("offset").with_type_hint(ArgumentTypeHint::Mixed),
            Argument::new("value").with_type_hint(ArgumentTypeHint::Mixed),
        ])
        .return_type(ReturnType::new(ReturnTypeHint::Void));

    class
        .add_method("offsetUnset", Visibility::Public, |this, arguments| {
            let offset = arguments[0].expect_long()?;
            let state = this.as_mut_state();
            state.array.remove(&offset);
            Ok::<_, phper::Error>(())
        })
        .argument(Argument::new("offset").with_type_hint(ArgumentTypeHint::Mixed))
        .return_type(ReturnType::new(ReturnTypeHint::Void));

    module.add_class(class);
}

fn integrate_i_bar(module: &mut Module) {
    let mut interface = InterfaceEntity::new(r"IntegrationTest\IBar");

    interface.extends(Interface::from_name("ArrayAccess"));
    interface.extends(Interface::from_name("Iterator"));

    interface
        .add_method("doSomethings")
        .argument(Argument::new("job_name"));

    module.add_interface(interface);
}

fn integrate_i_constants(module: &mut Module) {
    let mut interface = InterfaceEntity::new(r"IntegrationTest\IConstants");

    interface.add_constant("CST_STRING", "foo");
    interface.add_constant("CST_NULL", ());
    interface.add_constant("CST_TRUE", true);
    interface.add_constant("CST_FALSE", false);
    interface.add_constant("CST_INT", 100);
    interface.add_constant("CST_FLOAT", 10.0);

    module.add_interface(interface);
}

fn integrate_static_props(module: &mut Module) {
    let mut class = ClassEntity::new("IntegrationTest\\PropsHolder");

    class.add_static_property("foo", Visibility::Public, "bar");

    class.add_static_property("foo1", Visibility::Private, 12345i64);

    class.add_static_method("getFoo1", Visibility::Public, |_| {
        let val = ClassEntry::from_globals("IntegrationTest\\PropsHolder")?
            .get_static_property("foo1")
            .map(ToOwned::to_owned)
            .unwrap_or_default();
        phper::ok(val)
    });

    class
        .add_static_method("setFoo1", Visibility::Public, |params| {
            let foo1 = ClassEntry::from_globals("IntegrationTest\\PropsHolder")?
                .set_static_property("foo1", params[0].to_owned());
            phper::ok(foo1)
        })
        .argument(Argument::new("val"));

    module.add_class(class);
}

fn integrate_bar_extends_foo(module: &mut Module) {
    let mut cls = ClassEntity::new(r"IntegrationTest\BarExtendsFoo");
    cls.extends(r"IntegrationTest\Foo");
    cls.add_method("test", Visibility::Public, |_,_| {
        phper::ok(())
    });
    module.add_class(cls);
}

#[cfg(phper_major_version = "8")]
fn integrate_stringable(module: &mut Module) {
    use phper::{functions::ReturnType, types::ReturnTypeHint};

    let mut cls = ClassEntity::new(r"IntegrationTest\FooString");
    cls.implements(Interface::from_name("Stringable"));
    cls.add_method("__toString", Visibility::Public, |_this, _: &mut [ZVal]| {
        phper::ok("string")
    })
    .return_type(ReturnType::new(ReturnTypeHint::String));
    module.add_class(cls);
}
