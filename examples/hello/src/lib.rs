use phper::{echo, functions::Argument, modules::Module, php_get_module, values::ZVal};

/// The php function, receive arguments with type `ZVal`.
fn say_hello(arguments: &mut [ZVal]) -> phper::Result<()> {
    // Get the first argument, expect the type `ZStr`, and convert to Rust utf-8
    // str.
    let name = arguments[0].expect_z_str()?.to_str()?;

    // Macro which do php internal `echo`.
    echo!("Hello, {}!\n", name);

    Ok(())
}

/// This is the entry of php extension, the attribute macro `php_get_module`
/// will generate the `extern "C" fn`.
#[php_get_module]
pub fn get_module() -> Module {
    // New `Module` with extension info.
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Register function `say_hello`, with one argument `name`.
    module.add_function("say_hello", say_hello, vec![Argument::by_val("name")]);

    module
}
