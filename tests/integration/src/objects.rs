use phper::{classes::StatelessClassEntry, modules::Module, objects::Object, values::Val};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_objects_new_drop",
        |_: &mut [Val]| -> phper::Result<()> {
            let o = Object::new_by_std_class();
            drop(o);
            Ok(())
        },
        vec![],
    );

    module.add_function(
        "integrate_objects_get_set",
        |_: &mut [Val]| -> phper::Result<()> {
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

    module.add_function(
        "integrate_objects_set_val",
        |_: &mut [Val]| -> phper::Result<()> {
            let o = Object::new_by_std_class();
            let mut v = Val::null();
            v.set(o);
            Ok(())
        },
        vec![],
    );

    module.add_function(
        "integrate_objects_call",
        |_: &mut [Val]| -> phper::Result<()> {
            let o = StatelessClassEntry::from_globals("Exception")?.new_object();
            let message = o.call("getMessage", &[])?;
            dbg!(message.as_string()?);
            Ok(())
        },
        vec![],
    );
}
