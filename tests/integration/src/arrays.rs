use phper::{
    alloc::EBox,
    arrays::Array,
    functions::Argument,
    modules::Module,
    objects::Object,
    values::{SetVal, Val},
};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_arrays_new_drop",
        |arguments: &mut [Val]| -> phper::Result<String> {
            let mut a1 = Array::new();
            a1.insert("foo", Val::new("FOO"));
            let foo = a1.get("foo").unwrap();
            let foo = foo.as_string()?;

            let mut a2 = EBox::new(Array::new());
            a2.insert("bar", Val::new("BAR"));
            let bar = a2.get("bar").unwrap();
            let bar = bar.as_string()?;

            Ok(format!("{} {}", foo, bar))
        },
        vec![],
    );

    module.add_function(
        "integrate_arrays_types",
        |arguments: &mut [Val]| -> phper::Result<()> {
            let mut a = Array::new();

            a.insert(0, Val::new(0));
            a.insert(1, Val::new(1));
            a.insert("foo", Val::new("bar"));
            a.insert(
                "arr",
                Val::new({
                    let mut arr = Array::new();
                    arr.insert(0, Val::new(0));
                    arr.insert(1, Val::new(1));
                    EBox::new(arr)
                }),
            );
            a.insert(
                "obj",
                Val::new({
                    let mut o = Object::new_by_std_class();
                    o.set_property("foo", Val::new("bar"));
                    o
                }),
            );

            assert_eq!(a.get(0).unwrap().as_long()?, 0);
            assert_eq!(a.get(1).unwrap().as_long()?, 1);
            assert_eq!(a.get("foo").unwrap().as_string()?, "bar");

            let arr = a.get("arr").unwrap().as_array()?;
            assert_eq!(arr.get(0).unwrap().as_long()?, 0);
            assert_eq!(arr.get(1).unwrap().as_long()?, 1);

            let obj = a.get("obj").unwrap().as_object()?;
            let foo = obj.get_property("foo");
            assert_eq!(foo.as_string()?, "bar");

            assert!(a.get(10).is_none());
            assert!(a.get("not_exists").is_none());

            Ok(())
        },
        vec![],
    );
}
