use phper::{classes::DynamicClass, modules::Module, php_get_module};

pub mod http_client;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // let client = HttpClient::new();
    let client_class: DynamicClass<()> = DynamicClass::new();
    module.add_class("HttpClient", client_class);

    module
}
