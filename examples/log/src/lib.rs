use phper::{
    deprecated, echo, error, functions::Argument, modules::Module, notice, php_get_module,
    values::Val, warning,
};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));

    module.add_function(
        "log_say",
        |params: &mut [Val]| {
            let message = params[0].as_string();
            echo!("Hello, {}!", message);
        },
        vec![Argument::by_val("message")],
    );

    module.add_function(
        "log_notice",
        |params: &mut [Val]| {
            let message = params[0].as_string();
            notice!("Something happened: {}", message);
        },
        vec![Argument::by_val("message")],
    );

    module.add_function(
        "log_warning",
        |params: &mut [Val]| {
            let message = params[0].as_string();
            warning!("Something warning: {}", message);
        },
        vec![Argument::by_val("message")],
    );

    module.add_function(
        "log_error",
        |params: &mut [Val]| {
            let message = params[0].as_string();
            error!("Something gone failed: {}", message);
        },
        vec![Argument::by_val("message")],
    );

    module.add_function(
        "log_deprecated",
        |params: &mut [Val]| {
            let message = params[0].as_string();
            deprecated!("Something deprecated: {}", message);
        },
        vec![Argument::by_val("message")],
    );

    module
}
