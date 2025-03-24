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
        ClassEntity, Interface, InterfaceEntity, StateClass, Visibility,
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
    let _c_class = module.add_class(make_c_class(i_foo.clone()));
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
        });

    class
}

fn make_c_class(
    _i_foo: Interface,
) ->ClassEntity<()> {
    let mut class = ClassEntity::new(r"IntegrationTest\TypeHints\C");
    // String tests
    class
        .add_method("testString", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_str()?.to_str()?.to_string();
            phper::ok(())
        })
        .argument(Argument::by_val("string_value").with_type_hint(TypeHint::String));

    class
        .add_method("testStringOptional", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_str()?.to_str()?.to_string();
            phper::ok(())
        })
        .argument(Argument::by_val("string_value").with_type_hint(TypeHint::String).optional());

    class
        .add_method("testStringNullable", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_str()?.to_str()?.to_string();
            phper::ok(())
        })
        .argument(Argument::by_val("string_value").with_type_hint(TypeHint::String).nullable());

    // Bool tests
    class
        .add_method("testBool", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].as_bool();
            phper::ok(())
        })
        .argument(Argument::by_val("bool_value").with_type_hint(TypeHint::Bool));

    class
        .add_method("testBoolNullable", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].as_bool();
            phper::ok(())
        })
        .argument(Argument::by_val("bool_value").with_type_hint(TypeHint::Bool).nullable());

    class
        .add_method("testBoolOptional", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_bool()?;
            phper::ok(())
        })
        .argument(Argument::by_val("bool_value").with_type_hint(TypeHint::Bool).optional());

    // Int tests
    class
        .add_method("testInt", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_long()?;
            phper::ok(())
        })
        .argument(Argument::by_val("int_value").with_type_hint(TypeHint::Int));

    class
        .add_method("testIntNullable", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_long()?;
            phper::ok(())
        })
        .argument(Argument::by_val("int_value").with_type_hint(TypeHint::Int).nullable());

    class
        .add_method("testIntOptional", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_long()?;
            phper::ok(())
        })
        .argument(Argument::by_val("int_value").with_type_hint(TypeHint::Int).optional());

    // Float tests
    class
        .add_method("testFloat", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_double()?;
            phper::ok(())
        })
        .argument(Argument::by_val("float_value").with_type_hint(TypeHint::Float));

    class
        .add_method("testFloatOptional", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_double()?;
            phper::ok(())
        })
        .argument(Argument::by_val("float_value").with_type_hint(TypeHint::Float).optional());

    class
        .add_method("testFloatNullable", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_double()?;
            phper::ok(())
        })
        .argument(Argument::by_val("float_value").with_type_hint(TypeHint::Float).nullable());

    // Array tests
    class
        .add_method("testArray", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_arr()?;
            phper::ok(())
        })
        .argument(Argument::by_val("array_value").with_type_hint(TypeHint::Array));

    class
        .add_method("testArrayOptional", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_arr()?;
            phper::ok(())
        })
        .argument(Argument::by_val("array_value").with_type_hint(TypeHint::Array).optional());

    class
        .add_method("testArrayNullable", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_arr()?;
            phper::ok(())
        })
        .argument(Argument::by_val("array_value").with_type_hint(TypeHint::Array).nullable());

    // Mixed tests
    class
        .add_method("testMixed", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("mixed_value").with_type_hint(TypeHint::Mixed));

    // Callable tests
    class
        .add_method("testCallable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("callable_value").with_type_hint(TypeHint::Callable));

    class
        .add_method("testCallableNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("callable_value").with_type_hint(TypeHint::Callable).nullable());

    class
        .add_method("testCallableOptional", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("callable_value").with_type_hint(TypeHint::Callable).optional());

    class
        .add_method("testObject", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("object_value").with_type_hint(TypeHint::Object));

    class
        .add_method("testObjectNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("object_value").with_type_hint(TypeHint::Object).nullable());

    class
        .add_method("testObjectOptional", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("object_value").with_type_hint(TypeHint::Object).optional());

    class
        .add_method("testIterable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("iterable_value").with_type_hint(TypeHint::Iterable));

    class
        .add_method("testIterableNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("iterable_value").with_type_hint(TypeHint::Iterable).nullable());

    class
        .add_method("testIterableOptional", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("iterable_value").with_type_hint(TypeHint::Iterable).optional());

    class
        .add_method("testNull", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("null_value").with_type_hint(TypeHint::Null));

    // Class type test (assuming you have a classEntry for "DateTime")
    class
        .add_method("testClassEntry", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("classentry").with_type_hint(TypeHint::ClassEntry(String::from("todo"))));

    /*class
        .add_method("testClassNullable", Visibility::Public, move |_, arguments| {
            if arguments[0].is_null() {
                phper::ok("null")
            } else {
                let obj = arguments[0].expect_z_obj()?;
                let class_name = obj.get_class_name()?;
                phper::ok(class_name)
            }
        })
        .argument(Argument::by_val("datetime_obj").with_type_hint(TypeHint::ClassEntry(get_date_time_class_entry())).nullable());*/


    class
}