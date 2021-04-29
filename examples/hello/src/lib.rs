use phper::{
    arrays::Array,
    classes::StdClass,
    functions::Argument,
    ini::Policy,
    modules::{Module, ModuleArgs},
    objects::Object,
    php_get_module,
    values::{SetVal, Val},
};

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
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

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
        |this: &mut Object, _: &mut [Val]| {
            let prop = this.get_property("foo");
            Val::new(prop.as_string())
        },
        vec![],
    );
    foo_class.add_method(
        "setFoo",
        |this: &mut Object, arguments: &mut [Val]| {
            let prop = this.get_property("foo");
            // TODO add set_property method
            // prop.set(&mut arguments[0]);
        },
        vec![Argument::by_val("foo")],
    );
    module.add_class("FooClass", foo_class);

    module
}
