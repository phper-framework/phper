use phper::modules::Module;
use std::convert::Infallible;

pub fn integrate(module: &mut Module) {
    module.add_function("integrate_macros_globals", |_| {
        unsafe {
            let _compiler_options = phper::cg!(compiler_options);
            let _error_reporting = phper::eg!(error_reporting);
            let _connection_status = phper::pg!(connection_status);
            let _headers_sent = phper::sg!(headers_sent);
        }

        Ok::<_, Infallible>(())
    });
}
