# Register class

Registering classes is a bit more complicated than others.

First, you have to new the class build
[`ClassEntity`](phper::classes::ClassEntity), then register the parent class or
implements interfaces, add the properties and methods, finally add it into the
`Module`.

Here is the simplest example:

```rust,no_run
use phper::{classes::ClassEntity, modules::Module, php_get_module};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    let foo = ClassEntity::new("Foo");

    module.add_class(foo);

    module
}
```

Just like these codes in PHP:

```php
<?php

class Foo {}
```
