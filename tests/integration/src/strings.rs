use phper::{modules::Module, strings::ZendString, values::Val};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_strings_zend_string_new",
        |_: &mut [Val]| -> phper::Result<()> {
            let zs = ZendString::new("hello");
            assert_eq!(zs.as_str()?, "hello");

            let zs = ZendString::new([1, 2, 3]);
            assert_eq!(zs.as_ref(), &[1, 2, 3]);

            assert!(&*ZendString::new("hello") == &*ZendString::new(b"hello"));

            Ok(())
        },
        vec![],
    );
}
