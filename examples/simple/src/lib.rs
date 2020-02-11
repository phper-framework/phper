use phper::sys::{c_str, zend_execute_data, zend_module_entry, zval};
use phper::{
    php_function, php_get_module, Function, FunctionHandler, InternalArgInfo, InternalArgInfoArray,
    InternalBeginArgInfo, Module, Parameters, Value, ValueResult,
};

#[php_function]
fn zif_test_simple(parameters: Parameters) -> ValueResult {
    dbg!("zif_test_simple success");
    Ok(None)
}

#[php_get_module]
pub fn get_module() -> phper::Result<Module<'static>> {
    let arg_info = InternalArgInfoArray {
        begin: InternalBeginArgInfo {
            required_num_args: 2,
            ..Default::default()
        },
        parameters: vec![
            InternalArgInfo {
                name: c_str!("a"),
                ..Default::default()
            },
            InternalArgInfo {
                name: c_str!("b"),
                ..Default::default()
            },
        ],
    };

    let functions = vec![Function {
        name: c_str!("test_simple"),
        handler: FunctionHandler::Internal(zif_test_simple),
//        arg_info: Some(arg_info),
        ..Default::default()
    }];

    let module = Module {
        name: c_str!(env!("CARGO_PKG_NAME")),
        version: c_str!(env!("CARGO_PKG_VERSION")),
        functions: Some(functions),
        ..Default::default()
    };

    Ok(module)
}
