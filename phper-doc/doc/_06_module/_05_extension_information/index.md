# Extension information

By default, `PHPER` will auto register the `MINFO` handle, show the info about
extension name, version, authors, and display configuration items.

As you execute the command `php --ri <EXTENSION_NAME>`:

```txt
demo

version => 0.0.0
authors => PHPER Framework Team:jmjoy <jmjoy@apache.org>

Directive => Local Value => Master Value
complex.enable => 0 => 0
complex.foo => 100 => 100
```

If you want to add extra info items, you can use 
[`Module::add_info`](phper::modules::Module::add_info) method.

```rust,no_run
use phper::{modules::Module, php_get_module};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_info("extra info key", "extra info value");

    module
}
```

Then build the extension and call `php --ri <EXTENSION_NAME>`:

```txt
demo

version => 0.0.0
authors => PHPER Framework Team:jmjoy <jmjoy@apache.org>
extra info key => extra info value

Directive => Local Value => Master Value
complex.enable => 0 => 0
complex.foo => 100 => 100
```

The `extra info key` item is appeared.
