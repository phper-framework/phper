use phper::{modules::Module, objects::Object, values::Val};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_objects_new_drop",
        |arguments: &mut [Val]| -> phper::Result<()> {
            let o = Object::new_by_std_class();
            drop(o);

            Ok(())
        },
        vec![],
    );
}
