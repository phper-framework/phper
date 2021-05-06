#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]

mod arguments;
mod arrays;
mod objects;
mod strings;
mod values;

use phper::{modules::Module, php_get_module};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    arguments::integrate(&mut module);
    arrays::integrate(&mut module);
    objects::integrate(&mut module);
    values::integrate(&mut module);
    strings::integrate(&mut module);

    module
}
