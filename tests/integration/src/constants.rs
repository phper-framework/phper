use phper::modules::Module;

pub fn integrate(module: &mut Module) {
    module.add_constant("INTEGRATE_CONST_NULL", ());
    module.add_constant("INTEGRATE_CONST_TRUE", true);
    module.add_constant("INTEGRATE_CONST_FALSE", false);
    module.add_constant("INTEGRATE_CONST_LONG", 100i64);
    module.add_constant("INTEGRATE_CONST_DOUBLE", 200.);
    module.add_constant("INTEGRATE_CONST_STRING", "something");
    module.add_constant("INTEGRATE_CONST_BYTES", "something".as_bytes().to_owned());
}
