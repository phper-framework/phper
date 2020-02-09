use phper::sys::{c_str, zend_execute_data, zend_module_entry, zval};
use phper::{php_function, php_get_module, Function, Module, Parameters, Value, ValueResult};

#[php_function]
fn zif_test_simple(parameters: Parameters) -> ValueResult {
    dbg!("zif_test_simple success");
    Ok(None)
}

#[php_get_module]
pub fn get_module() -> phper::Result<Module<'static>> {
    let functions = vec![Function::builder()
        .name(c_str!("test_simple"))
        .handler(zif_test_simple)
        .build()
        .unwrap()];

    let module = Module::builder()
        .name(c_str!(env!("CARGO_PKG_NAME")))
        .version(c_str!(env!("CARGO_PKG_VERSION")))
        .functions(functions)
        .build()
        .unwrap();

    Ok(module)
}
