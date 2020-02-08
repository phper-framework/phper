use phper::sys::{c_str, zend_execute_data, zend_module_entry, zval};
use phper::{Function, Functions, Module};

fn zif_test_simple(_execute_data: *mut zend_execute_data, _return_value: *mut zval) {
    dbg!("zif_test_simple success");
}

#[no_mangle]
pub unsafe extern "C" fn get_module() -> *const zend_module_entry {
    let mut functions = Functions::new();
//    functions.push(Function::new(c_str!("test_simple"), &zif_test_simple));

    let module = Module::new(
        c_str!(env!("CARGO_PKG_NAME")),
        c_str!(env!("CARGO_PKG_VERSION")),
    )
    .functions(functions);

    module.into()
}
