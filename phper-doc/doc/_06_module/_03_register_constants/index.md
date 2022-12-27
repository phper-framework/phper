# Register constants

In `PHPER`, you can use [`add_constant`](phper::modules::Module::add_constant) to 
register constants.

```rust,no_run
use phper::{modules::Module, php_get_module};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_constant("FOO", 100i64);

    module
}
```

Because in PHP, you can also use copyable values as constants, such as long,
double and string, so the value have to implement [`Scalar`](phper::types::Scalar).
