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

    module.add_function(
        "integrate_objects_get_set",
        |arguments: &mut [Val]| -> phper::Result<()> {
            let mut o = Object::new_by_std_class();

            o.set_property("foo", Val::new("bar"));
            let foo = o.get_property("foo");
            assert_eq!(foo.as_string()?, "bar");

            let not_exists = o.get_property("no_exists");
            not_exists.as_null()?;

            Ok(())
        },
        vec![],
    );
}
