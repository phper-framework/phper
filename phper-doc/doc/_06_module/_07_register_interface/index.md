# Register interface

Registering interfaces is similar to registering classes.

First, you have to instantiate the class builder.
[`InterfaceEntity`](phper::classes::InterfaceEntity),
then extends interfaces (if there are),
add public abstract methods, finally add it into the `Module`.

Here is the simplest example:

```rust,no_run
use phper::{classes::InterfaceEntity, modules::Module, php_get_module};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    let foo = InterfaceEntity::new("Foo");

    module.add_interface(foo);

    module
}
```

Similarly in PHP:

```php
<?php

interface Foo {}
```

## Extends interfaces

If you want the interface `Foo` extends `ArrayAccess` and `Iterator` interfaces.

```rust,no_run
use phper::classes::{Interface, InterfaceEntity, ClassEntry};
use phper::classes::{array_access_class, iterator_class};

let mut foo = InterfaceEntity::new("Foo");
foo.extends(Interface::from_name("ArrayAccess"));
foo.extends(Interface::from_name("Iterator"));
```

Same as:

```php
<?php

interface Foo extends ArrayAccess, Iterator {}
```

## Add methods

Interface can add public abstract methods.

```rust,no_run
use phper::classes::{InterfaceEntity, ClassEntry, Visibility};
use phper::functions::Argument;
use phper::objects::StateObj;
use phper::values::ZVal;

let mut foo = InterfaceEntity::new("Foo");
foo.add_method("doSomethings").argument(Argument::new("name"));
```

Note that abstract has no method body, so you don't need to add the handler to the method.

## Add constants
Interfaces can have public constants. Value can be string|int|bool|float|null.

```rust,no_run
use phper::classes::InterfaceEntity;

let mut foo = InterfaceEntity::new("Foo");
foo.add_constant("ONE", "one");
foo.add_constant("TWO", 2);
foo.add_constant("THREE", 3.0);
```
