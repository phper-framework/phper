use phper::sys::{c_str, c_str_ptr, zend_execute_data, zend_module_entry, zval};
use phper::{
    php_function, php_get_module, Function, FunctionHandler, InternalArgInfo, InternalArgInfoArray,
    InternalBeginArgInfo, Module, Parameters, Value, ValueResult,
};
use std::os::raw::{c_char, c_int};
use std::ptr::null_mut;

#[php_function]
fn zif_test_simple(parameters: Parameters) -> ValueResult {
    dbg!(parameters.num_args());
    dbg!("zif_test_simple success");
    Ok(None)
}

// extern "C" fn zif_test_simple(
//     execute_data: *mut ::phper::sys::zend_execute_data,
//     return_value: *mut ::phper::sys::zval
// ) {
//     let a: *mut c_char = null_mut();
//     let a_len: *mut c_int = null_mut();
//     let b: *mut c_char = null_mut();
//     let b_len: *mut c_int = null_mut();

//     unsafe { phper::sys::zend_parse_parameters(2, c_str_ptr!("ss"), a, a_len, b, b_len); }
// }

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
        arg_info: Some(arg_info),
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
