use phper::{
    arrays::Array,
    classes::{DynamicClass, Visibility},
    functions::Argument,
    ini::{Ini, Policy},
    modules::{Module, ModuleContext},
    objects::Object,
    php_get_module,
    values::Val,
};

fn module_init(_args: ModuleContext) -> bool {
    true
}

fn say_hello(arguments: &mut [Val]) -> phper::Result<String> {
    let name = arguments[0].as_string_value()?;
    Ok(format!("Hello, {}!\n", name))
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
    Ini::add("hello.enable", false, Policy::All);
    Ini::add("hello.num", 100, Policy::All);
    Ini::add("hello.ratio", 1.5, Policy::All);
    Ini::add("hello.description", "hello world.".to_owned(), Policy::All);

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

            let hello_enable = Val::new(Ini::get::<bool>("hello.enable"));
            arr.insert("hello.enable", hello_enable);

            let hello_description = Val::new(Ini::get::<String>("hello.description"));
            arr.insert("hello.description", hello_description);

            arr
        },
        vec![],
    );

    // register classes
    let mut foo_class = DynamicClass::new("FooClass");
    foo_class.add_property("foo", Visibility::Private, 100);
    foo_class.add_method(
        "getFoo",
        Visibility::Public,
        |this: &mut Object<()>, _: &mut [Val]| {
            let prop = this.get_property("foo");
            Ok::<_, phper::Error>(prop.as_string_value()?)
        },
        vec![],
    );
    foo_class.add_method(
        "setFoo",
        Visibility::Public,
        |this: &mut Object<()>, arguments: &mut [Val]| -> phper::Result<()> {
            this.set_property("foo", Val::new(arguments[0].as_string_value()?));
            Ok(())
        },
        vec![Argument::by_val("foo")],
    );
    module.add_class(foo_class);

    module
}
