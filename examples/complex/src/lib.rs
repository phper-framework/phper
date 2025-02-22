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
    arrays::ZArray,
    classes::{ClassEntity, Visibility},
    functions::Argument,
    ini::{Policy, ini_get},
    modules::Module,
    objects::StateObj,
    php_get_module,
    values::ZVal,
};
use std::{convert::Infallible, ffi::CStr};

fn say_hello(arguments: &mut [ZVal]) -> phper::Result<String> {
    let name = &mut arguments[0];
    name.convert_to_string();
    let name = name.as_z_str().unwrap().to_str()?;
    Ok(format!("Hello, {}!\n", name))
}

fn throw_exception(_: &mut [ZVal]) -> phper::Result<()> {
    Err(phper::Error::Boxed("I am sorry".into()))
}

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // register module ini
    module.add_ini("complex.enable", false, Policy::All);
    module.add_ini("complex.num", 100, Policy::All);
    module.add_ini("complex.ratio", 1.5, Policy::All);
    module.add_ini(
        "complex.description",
        "hello world.".to_owned(),
        Policy::All,
    );

    // register hook functions
    module.on_module_init(|| {});
    module.on_module_shutdown(|| {});
    module.on_request_init(|| {});
    module.on_request_shutdown(|| {});

    // register functions
    module
        .add_function("complex_say_hello", say_hello)
        .argument(Argument::by_val("name"));
    module.add_function("complex_throw_exception", throw_exception);
    module.add_function("complex_get_all_ini", |_: &mut [ZVal]| {
        let mut arr = ZArray::new();

        let complex_enable = ZVal::from(ini_get::<bool>("complex.enable"));
        arr.insert("complex.enable", complex_enable);

        let complex_description = ZVal::from(ini_get::<Option<&CStr>>("complex.description"));
        arr.insert("complex.description", complex_description);
        Ok::<_, Infallible>(arr)
    });

    // register classes
    let mut foo_class = ClassEntity::new("FooClass");
    foo_class.add_property("foo", Visibility::Private, 100);
    foo_class.add_method(
        "getFoo",
        Visibility::Public,
        |this: &mut StateObj<()>, _: &mut [ZVal]| {
            let prop = this.get_property("foo");
            Ok::<_, phper::Error>(prop.clone())
        },
    );
    foo_class
        .add_method(
            "setFoo",
            Visibility::Public,
            |this: &mut StateObj<()>, arguments: &mut [ZVal]| -> phper::Result<()> {
                this.set_property("foo", arguments[0].clone());
                Ok(())
            },
        )
        .argument(Argument::by_val("foo"));
    module.add_class(foo_class);

    // register extra info
    module.add_info("extra info key", "extra info value");

    module
}
