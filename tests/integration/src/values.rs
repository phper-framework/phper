use phper::{modules::Module, values::Val};

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
        "integration_values_return_i32",
        integration_values_return_i32,
        vec![],
    );
    module.add_function(
        "integration_values_return_i64",
        integration_values_return_i64,
        vec![],
    );
}

fn integration_values_return_null(_: &mut [Val]) {}

fn integration_values_return_i32(_: &mut [Val]) -> i32 {
    32
}

fn integration_values_return_i64(_: &mut [Val]) -> i64 {
    64
}
