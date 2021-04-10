use std::{fs::OpenOptions, io::Write};

use phper::{
    c_str_ptr,
    classes::{Class, MethodEntity, StdClass, This},
    functions::create_zend_arg_info,
    ini::Policy,
    modules::{read_global_module, write_global_module, Module, ModuleArgs},
    php_function, php_get_module, php_minfo_function, php_minit_function, php_mshutdown_function,
    php_rinit_function, php_rshutdown_function,
    sys::{
        php_info_print_table_end, php_info_print_table_row, php_info_print_table_start,
        zend_function_entry, OnUpdateBool, PHP_INI_SYSTEM,
    },
    values::{ExecuteData, Val},
};

// static HELLO_ENABLE: ModuleGlobals<bool> = ModuleGlobals::new(false);
//
// static INI_ENTRIES: IniEntries<1> = IniEntries::new([HELLO_ENABLE.create_ini_entry(
//     "hello.enable",
//     "1",
//     Some(OnUpdateBool),
//     PHP_INI_SYSTEM,
// )]);
//
// #[php_minit_function]
// fn module_init(args: ModuleArgs) -> bool {
//     args.register_ini_entries(&INI_ENTRIES);
//     true
// }
//
// #[php_mshutdown_function]
// fn module_shutdown(args: ModuleArgs) -> bool {
//     args.unregister_ini_entries();
//     true
// }
//
// #[php_rinit_function]
// fn request_init(_args: ModuleArgs) -> bool {
//     true
// }
//
// #[php_rshutdown_function]
// fn request_shutdown(_args: ModuleArgs) -> bool {
//     true
// }
//
// #[php_minfo_function]
// fn module_info(module: &ModuleEntry) {
//     unsafe {
//         php_info_print_table_start();
//         php_info_print_table_row(2, c_str_ptr!("hello.version"), (*module.as_ptr()).version);
//         php_info_print_table_row(
//             2,
//             c_str_ptr!("hello.enable"),
//             if HELLO_ENABLE.get() {
//                 c_str_ptr!("1")
//             } else {
//                 c_str_ptr!("0")
//             },
//         );
//         php_info_print_table_end();
//     }
// }
//
// #[php_function]
// pub fn say_hello(execute_data: &mut ExecuteData) -> impl SetVal {
//     execute_data
//         .parse_parameters::<&str>()
//         .map(|name| format!("Hello, {}!", name))
// }
//
// static ARG_INFO_SAY_HELLO: MultiInternalArgInfo<1> =
//     MultiInternalArgInfo::new(1, false, [create_zend_arg_info(c_str_ptr!("n_ame"), false)]);
//
// static FUNCTION_ENTRIES: FunctionEntries<1> = FunctionEntries::new([zend_function_entry {
//     fname: c_str_ptr!("say_hello"),
//     handler: Some(say_hello),
//     arg_info: ARG_INFO_SAY_HELLO.as_ptr(),
//     num_args: 2,
//     flags: 0,
// }]);
//
// static MODULE_ENTRY: ModuleEntry = ModuleEntryBuilder::new(
//     c_str_ptr!(env!("CARGO_PKG_NAME")),
//     c_str_ptr!(env!("CARGO_PKG_VERSION")),
// )
// .functions(FUNCTION_ENTRIES.as_ptr())
// .module_startup_func(module_init)
// .module_shutdown_func(module_shutdown)
// .request_startup_func(request_init)
// .request_shutdown_func(request_shutdown)
// .info_func(module_info)
// .build();

// pub fn get_module() -> Module {
//     let mut module = Module::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
//     module.on_module_init(Box::new(|_| {
//         println!("Are you ok?");
//         true
//     }));
//     module
// }

fn module_init(_args: ModuleArgs) -> bool {
    // append_file("module_init");
    true
}

fn module_shutdown(_args: ModuleArgs) -> bool {
    // append_file("module_shutdown");
    true
}

fn request_init(_args: ModuleArgs) -> bool {
    // append_file("request_init");
    true
}

fn request_shutdown(_args: ModuleArgs) -> bool {
    // append_file("request_shutdown");
    true
}

fn append_file(s: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/tmp/hello")
        .unwrap();

    writeln!(file, "{}", s).unwrap();
}

fn test_func() {}

#[no_mangle]
pub extern "C" fn get_module() -> *const ::phper::sys::zend_module_entry {
    write_global_module(|module| {
        module.set_name(env!("CARGO_PKG_NAME"));
        module.set_version(env!("CARGO_PKG_VERSION"));

        module.add_bool_ini("hello.enable", false, Policy::All);
        module.add_long_ini("hello.len", 100, Policy::All);
        module.add_real_ini("hello.ratio", 1.5, Policy::All);
        module.add_str_ini("hello.description", "empty", Policy::All);

        module.on_module_init(module_init);
        module.on_module_shutdown(module_shutdown);
        module.on_request_init(request_init);
        module.on_request_shutdown(request_shutdown);

        module.add_function("hello_fuck", || {
            let hello_enable = Module::get_bool_ini("hello.enable");
            dbg!(hello_enable);

            let hello_description = Module::get_str_ini("hello.description");
            dbg!(hello_description);
        });
        module.add_function("test_func", test_func);

        let mut std_class = StdClass::new();
        std_class.add_property("foo", 100);
        std_class.add_method("test1", |_: &mut This| {
            println!("hello test1");
        });
        module.add_class("Test1", std_class);
    });

    unsafe {
        read_global_module(|module| {
            module.module_entry()
        })
    }
}
