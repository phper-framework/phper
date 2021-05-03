use phper::{
    deprecated, echo, error, functions::Argument, modules::Module, notice, php_get_module,
    values::Val, warning,
};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_function(
        "log_say",
        |params: &mut [Val]| -> phper::Result<()> {
            let message = params[0].as_string_value()?;
            echo!("Hello, {}!", message);
            Ok(())
        },
        vec![Argument::by_val("message")],
    );

    module.add_function(
        "log_notice",
        |params: &mut [Val]| -> phper::Result<()> {
            let message = params[0].as_string_value()?;
            notice!("Something happened: {}", message);
            Ok(())
        },
        vec![Argument::by_val("message")],
    );

    module.add_function(
        "log_warning",
        |params: &mut [Val]| -> phper::Result<()> {
            let message = params[0].as_string_value()?;
            warning!("Something warning: {}", message);
            Ok(())
        },
        vec![Argument::by_val("message")],
    );

    module.add_function(
        "log_error",
        |params: &mut [Val]| -> phper::Result<()> {
            let message = params[0].as_string_value()?;
            error!("Something gone failed: {}", message);
            Ok(())
        },
        vec![Argument::by_val("message")],
    );

    module.add_function(
        "log_deprecated",
        |params: &mut [Val]| -> phper::Result<()> {
            let message = params[0].as_string_value()?;
            deprecated!("Something deprecated: {}", message);
            Ok(())
        },
        vec![Argument::by_val("message")],
    );

    module
}
