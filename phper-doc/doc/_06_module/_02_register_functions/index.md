# Register functions

In `PHPER`, you can use [`add_function`](phper::modules::Module::add_function) to 
register functions.

```rust,no_run
use phper::{modules::Module, php_get_module, functions::Argument, echo};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_function("say_hello", |arguments| -> phper::Result<()> {
        let name = arguments[0].expect_z_str()?.to_str()?;
        echo!("Hello, {}!\n", name);
        Ok(())
    }).argument(Argument::new("name"));

    module
}
```

This example registers a function called `say_hello` and accepts a parameter 
`name` passed by value, similarly in PHP:

```php
<?php

function say_hello($name) {
    echo "Hello, {$name}\n";
}
```

You can get the function info by `php --re <EXTENSION_NAME>`:

```txt
Extension [ <persistent> extension #56 hello version <no_version> ] {

  - Functions {
    Function [ <internal:hello> function say_hello ] {

      - Parameters [1] {
        Parameter #0 [ <required> $name ]
      }
    }
  }
}
```

It is useful to register the parameters of the function, which can limit the 
number of parameters of the function.

Especially when the parameters need to be passed by reference.

```rust,no_run
use phper::{modules::Module, php_get_module, functions::Argument};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_function("add_count", |arguments| -> phper::Result<()> {
        let count = arguments[0].expect_mut_z_ref()?;
        *count.val_mut().expect_mut_long()? += 100;
        Ok(())
    }).argument(Argument::new("count").by_ref());

    module
}
```

Here, the argument is registered as
[`Argument::by_ref`](phper::functions::Argument::by_ref).  Therefore, the type of
the `count` parameter is no longer long, but a reference.

## Argument and return type modifiers

Arguments can have type-hints, nullability and default values applied. Here we define a function that accepts
a nullable class (in this case, an interface), and a string with a default value:

```rust,no_run
use phper::{modules::Module, php_get_module, functions::Argument, echo};
use phper::types::ArgumentTypeHint;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_function("my_function", |_| -> phper::Result<()> {
        Ok(())
    })
    .argument(Argument::new("a_class").with_type_hint(ArgumentTypeHint::ClassEntry(String::from(r"\MyNamespace\MyInterface"))).allow_null())
    .argument(Argument::new("name").with_type_hint(ArgumentTypeHint::String).with_default_value("'my_default'"))
    .argument(Argument::new("optional_bool").with_type_hint(ArgumentTypeHint::Bool).optional());

    module
}
```

The output of `php --re` for this function would look like:

```txt
    Function [ <internal:integration> function my_function ] {

      - Parameters [3] {
        Parameter #0 [ <required> ?class_name $a_class ]
        Parameter #1 [ <optional> string $name = 'my_default' ]
        Parameter #2 [ <optional> bool $optional_bool = <default> ]
      }
    }
```
