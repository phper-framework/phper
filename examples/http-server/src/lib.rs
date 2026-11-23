use crate::{
    errors::make_exception_class, request::make_request_class, response::make_response_class,
    server::make_server_class,
};
use phper::{modules::Module, php_get_module};

pub mod errors;
pub mod request;
pub mod response;
pub mod server;

#[php_get_module]
pub fn get_module() -> Module {
    // Add module info.
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Register classes.
    module.add_class(make_exception_class());
    let request_class = module.add_class(make_request_class());
    let response_class = module.add_class(make_response_class());
    module.add_class(make_server_class(request_class, response_class));

    module
}
