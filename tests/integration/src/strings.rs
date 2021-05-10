use phper::{modules::Module, strings::ZendString, values::Val};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_strings_as_string",
        |_: &mut [Val]| -> phper::Result<()> {
            let zs = ZendString::new("hello");
            assert_eq!(zs.to_string()?, "hello");
            Ok(())
        },
        vec![],
    );
}
