use phper::{arrays::Array, functions::call, modules::Module, values::Val};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_functions_call",
        |_: &mut [Val]| -> phper::Result<()> {
            let mut arr = Array::new();
            arr.insert("a", Val::new(1));
            arr.insert("b", Val::new(2));
            let ret = call("json_encode", &[Val::new(arr)])?;
            assert_eq!(ret.as_string()?, r#"{"a":1,"b":2}"#);
            Ok(())
        },
        vec![],
    );
}
