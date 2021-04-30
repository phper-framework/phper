use crate::http_client::HttpClient;
use phper::{classes::StdClass, modules::Module, php_get_module};

pub mod http_client;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    let client = HttpClient::new();
    let client_class = StdClass::new();
    module.add_class("HttpClient", client_class);

    module
}
