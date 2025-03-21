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
    classes::{
        ClassEntity, ClassEntry, Interface, InterfaceEntity, StateClass, Visibility,
    },
    functions::{Argument, ReturnType},
    modules::Module,
    types::{TypeHint, TypeInfo},
    values::ZVal,
};

pub fn integrate(module: &mut Module) {
    let i_foo = module.add_interface(make_i_foo_interface());
    let a_class = module.add_class(make_foo_class(i_foo.clone()));
    let _b_class = module.add_class(make_b_class(a_class.clone(), i_foo.clone()));
    let _c_class = module.add_class(make_c_class());
}

fn make_i_foo_interface() -> InterfaceEntity {
    let mut interface = InterfaceEntity::new(r"IntegrationTest\TypeHints\IFoo");
    interface.add_method("getValue")
        .return_type(ReturnType::by_val(TypeInfo::STRING));
    interface.add_method("setValue")
        .argument(Argument::by_val("foo"));

    interface
}

fn make_foo_class(
    i_foo: Interface,
) -> ClassEntity<()> {
    let mut class = ClassEntity::new(r"IntegrationTest\TypeHints\Foo");

    //leak Interface so that ClassEntry can be retrieved later, during module startup
    let i_foo_copy: &'static Interface = Box::leak(Box::new(i_foo));
    class.implements(move || {
        i_foo_copy.as_class_entry()
    });
    class
        .add_method("getValue", Visibility::Public, |this,_| {
            let value = this
                .get_property("value")
                .expect_z_str()?
                .to_str()?
                .to_owned();
            Ok::<_, phper::Error>(value)
        })
        .return_type(ReturnType::by_val(TypeInfo::STRING));

    class
        .add_method("setValue", Visibility::Public, |this, arguments| {
            let name = arguments[0].expect_z_str()?.to_str()?;
            this.set_property("value", ZVal::from(name));
            Ok::<_, phper::Error>(())
        })
        .argument(Argument::by_val("foo"));

    class.add_property("value", Visibility::Private, "");

    class
}

fn make_b_class(
    a_class: StateClass<()>,
    i_foo: Interface,
) -> ClassEntity<()> {
    let mut class = ClassEntity::new(r"IntegrationTest\TypeHints\B");
    let _i_foo_copy: &'static Interface = Box::leak(Box::new(i_foo));

    class
        .add_static_method("createFoo", Visibility::Public, move |_| {
            let object = a_class.init_object()?;
            Ok::<_, phper::Error>(object)
        }); //todo return ClassEntity(i_foo)

    class
}

fn make_c_class() ->ClassEntity<()> {
    let mut class = ClassEntity::new(r"IntegrationTest\TypeHints\C");
    class
        .add_method("exString", Visibility::Public, move |_, arguments| {
            let name = arguments[0].expect_z_str()?.to_str()?.to_string();
            phper::ok(name)
        })
        .argument(Argument::by_val("string_value").with_type_hint(TypeHint::String));

    class
        .add_method("exStringOptional", Visibility::Public, move |_, arguments| {
            let name = arguments[0].expect_z_str()?.to_str()?.to_string();
            phper::ok(name)
        })
        .argument(Argument::by_val("string_value").with_type_hint(TypeHint::String).nullable());

    class
        .add_method("exBool", Visibility::Public, move |_, arguments| {
            let name = arguments[0].as_bool();
            phper::ok(name)
        })
        .argument(Argument::by_val("bool_value").with_type_hint(TypeHint::Bool));

    class
        .add_method("exBoolOptional", Visibility::Public, move |_, arguments| {
            let name = arguments[0].as_bool();
            phper::ok(name)
        })
        .argument(Argument::by_val("bool_value").with_type_hint(TypeHint::Bool).nullable());


    class
}