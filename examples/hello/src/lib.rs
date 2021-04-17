use phper::{
    arrays::Array,
    c_str_ptr,
    classes::{Class, StdClass, This},
    functions::{create_zend_arg_info, Argument},
    ini::Policy,
    modules::{read_global_module, write_global_module, Module, ModuleArgs},
    php_get_module,
    sys::{
        php_info_print_table_end, php_info_print_table_row, php_info_print_table_start,
        zend_function_entry, OnUpdateBool, PHP_INI_SYSTEM,
    },
    values::{ExecuteData, SetVal, Val},
    Throwable,
};
use std::{fs::OpenOptions, io::Write};

fn module_init(_args: ModuleArgs) -> bool {
    true
}

fn say_hello(arguments: &mut [Val]) -> impl SetVal {
    let name = arguments[0].as_string();
    format!("Hello, {}!\n", name)
}

fn throw_exception(_: &mut [Val]) -> phper::Result<()> {
    Err(phper::Error::other("I am sorry"))
}

#[php_get_module]
pub extern "C" fn get_module(module: &mut Module) {
    // set module metadata
    module.set_name(env!("CARGO_PKG_NAME"));
    module.set_version(env!("CARGO_PKG_VERSION"));
    module.set_author(env!("CARGO_PKG_AUTHORS"));

    // register module ini
    module.add_bool_ini("hello.enable", false, Policy::All);
    module.add_long_ini("hello.num", 100, Policy::All);
    module.add_real_ini("hello.ratio", 1.5, Policy::All);
    module.add_str_ini("hello.description", "hello world.", Policy::All);

    // register hook functions
    module.on_module_init(module_init);
    module.on_module_shutdown(|_| true);
    module.on_request_init(|_| true);
    module.on_request_shutdown(|_| true);

    // register functions
    module.add_function("hello_say_hello", say_hello, vec![Argument::by_val("name")]);
    module.add_function("hello_throw_exception", throw_exception, vec![]);
    module.add_function(
        "hello_get_all_ini",
        |_: &mut [Val]| {
            let mut arr = Array::new();

            let mut hello_enable = Val::new(Module::get_bool_ini("hello.enable"));
            arr.insert("hello.enable", &mut hello_enable);

            let mut hello_description = Val::new(Module::get_str_ini("hello.description"));
            arr.insert("hello.description", &mut hello_description);

            arr
        },
        vec![],
    );

    // register classes
    let mut foo_class = StdClass::new();
    foo_class.add_property("foo", 100);
    foo_class.add_method(
        "getFoo",
        |this: &mut This, _: &mut [Val]| {
            let prop = this.get_property("foo");
            Val::from_val(prop)
        },
        vec![],
    );
    foo_class.add_method(
        "setFoo",
        |this: &mut This, arguments: &mut [Val]| {
            let prop = this.get_property("foo");
            prop.set(&arguments[0]);
        },
        vec![Argument::by_val("foo")],
    );
    module.add_class("FooClass", foo_class);
}