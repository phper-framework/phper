use phper::{functions::Argument, modules::Module, values::Val};

pub fn integrate(module: &mut Module) {
    integrate_arguments(module);
}

fn integrate_arguments(module: &mut Module) {
    module.add_function(
        "integrate_arguments_null",
        |arguments: &mut [Val]| arguments[0].as_null(),
        vec![Argument::by_val("n")],
    );
}
