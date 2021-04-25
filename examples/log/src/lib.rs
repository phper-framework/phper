use phper::{
    deprecated, error, functions::Argument, modules::Module, notice, php_get_module, values::Val,
    warning,
};

#[php_get_module]
pub fn get_module(module: &mut Module) {
    // set module metadata
    module.set_name(env!("CARGO_PKG_NAME"));
    module.set_version(env!("CARGO_PKG_VERSION"));
    module.set_author(env!("CARGO_PKG_AUTHORS"));

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
}
