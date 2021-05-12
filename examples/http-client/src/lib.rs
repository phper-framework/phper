use crate::client::make_client_class;
use anyhow::Context;
use phper::{classes::DynamicClass, modules::Module, php_get_module};

pub mod client;
pub mod errors;
pub mod request;
pub mod response;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_class(make_client_class());

    module
}
