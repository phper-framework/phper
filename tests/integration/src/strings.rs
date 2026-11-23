use phper::{modules::Module, strings::ZString, values::ZVal};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_strings_zend_string_new",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let zs = ZString::new("hello");
            assert_eq!(zs.to_str()?, "hello");

            let zs = ZString::new([1, 2, 3]);
            assert_eq!(AsRef::<[u8]>::as_ref(&zs), &[1, 2, 3]);

            assert!(ZString::new("hello") == ZString::new(b"hello"));

            Ok(())
        },
    );

    module.add_function(
        "integrate_strings_zend_string_new_persistent",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let zs = ZString::new_persistent("persistent_hello");
            assert_eq!(zs.to_str()?, "persistent_hello");

            let zs = ZString::new_persistent([4, 5, 6]);
            assert_eq!(AsRef::<[u8]>::as_ref(&zs), &[4, 5, 6]);

            assert!(
                ZString::new_persistent("persistent") == ZString::new_persistent(b"persistent")
            );

            Ok(())
        },
    );
}
