use phper::sys::{zend_module_entry, zend_execute_data, zval};
use phper::module::Module;
use phper::sys::c_str;
use phper::function::Function;

#[no_mangle]
pub extern "C" fn zif_simple(execute_data: *mut zend_execute_data, return_value: *mut zval) {

}

#[no_mangle]
pub unsafe extern "C" fn get_module() -> *const zend_module_entry {
    let module = Module::new(c_str!("simple"));
//        .functions(vec![Function::new(c_str!("simple"), zif_simple)]);
    module.into()
}
