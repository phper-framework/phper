use phper::{arrays::Array, modules::Module, values::Val};

pub fn integrate(module: &mut Module) {
    integrate_returns(module);
}

fn integrate_returns(module: &mut Module) {
    module.add_function(
        "integration_values_return_null",
        integration_values_return_null,
        vec![],
    );
    module.add_function(
        "integration_values_return_true",
        integration_values_return_true,
        vec![],
    );
    module.add_function(
        "integration_values_return_false",
        integration_values_return_false,
        vec![],
    );
    module.add_function(
        "integration_values_return_i32",
        integration_values_return_i32,
        vec![],
    );
    module.add_function(
        "integration_values_return_u32",
        integration_values_return_u32,
        vec![],
    );
    module.add_function(
        "integration_values_return_i64",
        integration_values_return_i64,
        vec![],
    );
    module.add_function(
        "integration_values_return_f64",
        integration_values_return_f64,
        vec![],
    );
    module.add_function(
        "integration_values_return_str",
        integration_values_return_str,
        vec![],
    );
    module.add_function(
        "integration_values_return_string",
        integration_values_return_string,
        vec![],
    );
    module.add_function(
        "integration_values_return_array",
        integration_values_return_array,
        vec![],
    );
}

fn integration_values_return_null(_: &mut [Val]) {}

fn integration_values_return_true(_: &mut [Val]) -> bool {
    true
}

fn integration_values_return_false(_: &mut [Val]) -> bool {
    false
}

fn integration_values_return_i32(_: &mut [Val]) -> i32 {
    32
}

fn integration_values_return_u32(_: &mut [Val]) -> u32 {
    32
}

fn integration_values_return_i64(_: &mut [Val]) -> i64 {
    64
}

fn integration_values_return_f64(_: &mut [Val]) -> f64 {
    64.0
}

fn integration_values_return_str(_: &mut [Val]) -> &'static str {
    "foo"
}

fn integration_values_return_string(_: &mut [Val]) -> String {
    "foo".to_string()
}

fn integration_values_return_array(_: &mut [Val]) -> Array {
    let mut arr = Array::new();
    arr.insert("a", Val::new(1));
    arr.insert("b", Val::new("foo"));
    arr
}
