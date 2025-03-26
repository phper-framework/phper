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
    types::{ArgumentTypeHint, ReturnTypeHint},
    values::ZVal,
};

const I_FOO: &str = r"IntegrationTest\TypeHints\IFoo";

pub fn integrate(module: &mut Module) {
    let i_foo = module.add_interface(make_i_foo_interface());
    let foo_class = module.add_class(make_foo_class(i_foo.clone()));
    let _ = module.add_class(make_b_class(foo_class.clone(), i_foo.clone()));
    let _ = module.add_class(make_foo_handler());
    let _ = module.add_class(make_arg_typehint_class());
    let _ = module.add_class(make_return_typehint_class());
}

fn make_i_foo_interface() -> InterfaceEntity {
    let mut interface = InterfaceEntity::new(r"IntegrationTest\TypeHints\IFoo");
    interface.add_method("getValue")
        .return_type(ReturnType::by_val(ReturnTypeHint::String));
    interface.add_method("setValue")
        .argument(Argument::by_val("value"));

    interface
}

fn make_foo_handler() -> ClassEntity<()> {
    let mut class = ClassEntity::new(r"IntegrationTest\TypeHints\FooHandler");

    class.add_method("handle", Visibility::Public, |_,arguments| {
        phper::ok(arguments[0].clone())
    })
        .argument(Argument::by_val("foo").with_type_hint(ArgumentTypeHint::ClassEntry(String::from(I_FOO))))
        .return_type(ReturnType::by_val(ReturnTypeHint::ClassEntry(String::from(I_FOO))));

    class
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
        .return_type(ReturnType::by_val(ReturnTypeHint::String));

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

fn make_arg_typehint_class() ->ClassEntity<()> {
    let mut class = ClassEntity::new(r"IntegrationTest\TypeHints\ArgumentTypeHintTest");
    // String tests
    class
        .add_method("testString", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_str()?.to_str()?.to_string();
            phper::ok(())
        })
        .argument(Argument::by_val("string_value").with_type_hint(ArgumentTypeHint::String));

    class
        .add_method("testStringOptional", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_str()?.to_str()?.to_string();
            phper::ok(())
        })
        .argument(Argument::by_val("string_value").with_type_hint(ArgumentTypeHint::String).optional());

    class
        .add_method("testStringNullable", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_str()?.to_str()?.to_string();
            phper::ok(())
        })
        .argument(Argument::by_val("string_value").with_type_hint(ArgumentTypeHint::String).nullable());

    // Bool tests
    class
        .add_method("testBool", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].as_bool();
            phper::ok(())
        })
        .argument(Argument::by_val("bool_value").with_type_hint(ArgumentTypeHint::Bool));

    class
        .add_method("testBoolNullable", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].as_bool();
            phper::ok(())
        })
        .argument(Argument::by_val("bool_value").with_type_hint(ArgumentTypeHint::Bool).nullable());

    class
        .add_method("testBoolOptional", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_bool()?;
            phper::ok(())
        })
        .argument(Argument::by_val("bool_value").with_type_hint(ArgumentTypeHint::Bool).optional());

    // Int tests
    class
        .add_method("testInt", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_long()?;
            phper::ok(())
        })
        .argument(Argument::by_val("int_value").with_type_hint(ArgumentTypeHint::Int));

    class
        .add_method("testIntNullable", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_long()?;
            phper::ok(())
        })
        .argument(Argument::by_val("int_value").with_type_hint(ArgumentTypeHint::Int).nullable());

    class
        .add_method("testIntOptional", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_long()?;
            phper::ok(())
        })
        .argument(Argument::by_val("int_value").with_type_hint(ArgumentTypeHint::Int).optional());

    // Float tests
    class
        .add_method("testFloat", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_double()?;
            phper::ok(())
        })
        .argument(Argument::by_val("float_value").with_type_hint(ArgumentTypeHint::Float));

    class
        .add_method("testFloatOptional", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_double()?;
            phper::ok(())
        })
        .argument(Argument::by_val("float_value").with_type_hint(ArgumentTypeHint::Float).optional());

    class
        .add_method("testFloatNullable", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_double()?;
            phper::ok(())
        })
        .argument(Argument::by_val("float_value").with_type_hint(ArgumentTypeHint::Float).nullable());

    // Array tests
    class
        .add_method("testArray", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_arr()?;
            phper::ok(())
        })
        .argument(Argument::by_val("array_value").with_type_hint(ArgumentTypeHint::Array));

    class
        .add_method("testArrayOptional", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_arr()?;
            phper::ok(())
        })
        .argument(Argument::by_val("array_value").with_type_hint(ArgumentTypeHint::Array).optional());

    class
        .add_method("testArrayNullable", Visibility::Public, move |_, arguments| {
            let _ = arguments[0].expect_z_arr()?;
            phper::ok(())
        })
        .argument(Argument::by_val("array_value").with_type_hint(ArgumentTypeHint::Array).nullable());

    // Mixed tests
    class
        .add_method("testMixed", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("mixed_value").with_type_hint(ArgumentTypeHint::Mixed));

    // Callable tests
    class
        .add_method("testCallable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("callable_value").with_type_hint(ArgumentTypeHint::Callable));

    class
        .add_method("testCallableNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("callable_value").with_type_hint(ArgumentTypeHint::Callable).nullable());

    class
        .add_method("testCallableOptional", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("callable_value").with_type_hint(ArgumentTypeHint::Callable).optional());

    class
        .add_method("testObject", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("object_value").with_type_hint(ArgumentTypeHint::Object));

    class
        .add_method("testObjectNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("object_value").with_type_hint(ArgumentTypeHint::Object).nullable());

    class
        .add_method("testObjectOptional", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("object_value").with_type_hint(ArgumentTypeHint::Object).optional());

    class
        .add_method("testIterable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("iterable_value").with_type_hint(ArgumentTypeHint::Iterable));

    class
        .add_method("testIterableNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("iterable_value").with_type_hint(ArgumentTypeHint::Iterable).nullable());

    class
        .add_method("testIterableOptional", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("iterable_value").with_type_hint(ArgumentTypeHint::Iterable).optional());

    class
        .add_method("testNull", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("null_value").with_type_hint(ArgumentTypeHint::Null));

    class
        .add_method("testClassEntry", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("classentry").with_type_hint(ArgumentTypeHint::ClassEntry(String::from(I_FOO))));

    class
        .add_method("testClassEntryNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("classentry").with_type_hint(ArgumentTypeHint::ClassEntry(String::from(I_FOO))).nullable());

    class
        .add_method("testClassEntryOptional", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .argument(Argument::by_val("classentry").with_type_hint(ArgumentTypeHint::ClassEntry(String::from(I_FOO))).optional());

    class
}

fn make_return_typehint_class() ->ClassEntity<()> {
    let mut class = ClassEntity::new(r"IntegrationTest\TypeHints\ReturnTypeHintTest");
    class
        .add_method("returnNull", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Null));

    class
        .add_method("returnString", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::String));

    class
        .add_method("returnStringNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::String).allow_null());

    class
        .add_method("returnBool", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Bool));

    class
        .add_method("returnBoolNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Bool).allow_null());

    class
        .add_method("returnInt", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Int));

    class
        .add_method("returnIntNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Int).allow_null());

    class
        .add_method("returnFloat", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Float));

    class
        .add_method("returnFloatNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Float).allow_null());

    class
        .add_method("returnArray", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Array));

    class
        .add_method("returnArrayNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Array).allow_null());

    class
        .add_method("returnObject", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Object));

    class
        .add_method("returnObjectNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Object).allow_null());

    class
        .add_method("returnCallable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Callable));

    class
        .add_method("returnCallableNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Callable).allow_null());

    class
        .add_method("returnIterable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Iterable));

    class
        .add_method("returnIterableNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Iterable).allow_null());

    class
        .add_method("returnMixed", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Mixed));

    class
        .add_method("returnClassEntry", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::ClassEntry(String::from(I_FOO))));

    class
        .add_method("returnClassEntryNullable", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::ClassEntry(String::from(I_FOO))).allow_null());

    class
        .add_method("returnNever", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Never));

    class
        .add_method("returnVoid", Visibility::Public, move |_, _| {
            phper::ok(())
        })
        .return_type(ReturnType::by_val(ReturnTypeHint::Void));

    class
}