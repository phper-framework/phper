use phper::{
    c_str_ptr,
    classes::{Class, MethodEntity, StdClass, This},
    functions::{create_zend_arg_info, Argument},
    ini::Policy,
    modules::{read_global_module, write_global_module, Module, ModuleArgs},
    php_get_module,
    sys::{
        php_info_print_table_end, php_info_print_table_row, php_info_print_table_start,
        zend_function_entry, OnUpdateBool, PHP_INI_SYSTEM,
    },
    throws::{Exception, Throwable},
    values::{ExecuteData, Val},
};
use std::{fs::OpenOptions, io::Write};

fn module_init(_args: ModuleArgs) -> bool {
    true
}

fn say_hello(arguments: &mut [Val]) -> String {
    let name = arguments[0].as_string();
    format!("Hello, {}\n", name)
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
    module.add_function(
        "hello_get_all_ini",
        |_: &mut [Val]| -> Result<String, Exception> {
            let hello_enable = Module::get_bool_ini("hello.enable");
            dbg!(hello_enable);

            let hello_description = Module::get_str_ini("hello.description");
            dbg!(hello_description);

            Ok(String::new())
        },
        vec![],
    );

    // register classes
    let mut std_class = StdClass::new();
    std_class.add_property("foo", 100);
    std_class.add_method("test1", |_: &mut This| {
        println!("hello test1");
    });
    module.add_class("Test1", std_class);
}
