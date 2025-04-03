# Register class

Registering classes is a bit more complicated than others.

First, you have to instantiate the class builder
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

Similarly in PHP:

```php
<?php

class Foo {}
```

## Extends & implements

To allow a class to extend another, you can use `ClassEntity.extends(StateClass<T>)`.
A StateClass can either be obtained by registering your own class against the module, or
by  looking up the class by name (for example, for PHP built-in classes like `Exception`).

If you want class `Foo` extends `Bar`, and implements interface `Stringable`:

```rust,no_run
use phper::classes::{ClassEntity, ClassEntry, Interface};
use phper::{modules::Module, php_get_module};

let mut module = Module::new("test", "0.1", "");

let bar = module.add_class(ClassEntity::new("Bar"));
let mut foo = ClassEntity::new("Foo");
foo.extends(bar);
foo.implements(Interface::from_name("Stringable"));
```

You should implement the method `Stringable::__toString` after implementing
`Stringable`, otherwise the class `Foo` will become abstract class.

## Add properties

Due to the limitation of PHP, you can only use copyable values as class properties.

```rust,no_run
use phper::classes::{ClassEntity, ClassEntry, Visibility};

let mut foo = ClassEntity::new("Foo");
foo.add_property("prop", Visibility::Public, "the prop value");
```

## Add methods

Adding class methods is similar with adding module functions, the difference is that
adding class methods increases `Visibility` and `$this` object.

```rust,no_run
use phper::classes::{ClassEntity, ClassEntry, Visibility};
use phper::objects::StateObj;
use phper::values::ZVal;

let mut foo = ClassEntity::new("Foo");
foo.add_method(
    "getProp",
    Visibility::Public,
    |this: &mut StateObj<()>, _: &mut [ZVal]| {
        let prop = this.get_property("foo");
        Ok::<_, phper::Error>(prop.clone())
    },
);
```

Adding static class methods is more like adding module functions, because there is no
`$this` variable.

```rust,no_run
use phper::classes::{ClassEntity, ClassEntry, Visibility};
use phper::functions::Argument;
use phper::values::ZVal;

let mut foo = ClassEntity::new("Foo");
foo.add_static_method(
    "staticSayHello",
    Visibility::Public,
    |arguments: &mut [ZVal]| {
        let name = arguments[0].expect_z_str()?.to_str()?;
        Ok::<_, phper::Error>(format!("Hello, {}!\n", name))
    },
).argument(Argument::new("name"));
```

## Argument and return type modifiers

Methods may add argument and return typehints as per functions. For example:

```rust,no_run
use phper::classes::{ClassEntity, ClassEntry, Visibility};
use phper::functions::{Argument, ReturnType};
use phper::types::{ArgumentTypeHint, ReturnTypeHint};

let mut foo = ClassEntity::new("Foo");
foo.add_method(
    "test",
    Visibility::Public,
    |_this, _arguments| -> phper::Result<()> {
        Ok(())
    },
)
.argument(Argument::new("a_string").with_type_hint(ArgumentTypeHint::String))
.argument(Argument::new("an_interface").with_type_hint(ArgumentTypeHint::ClassEntry(String::from(r"\MyNamespace\MyInterface"))))
.return_type(ReturnType::new(ReturnTypeHint::Bool).allow_null());
```

## Add constants
Interfaces can have public constants. Value can be string|int|bool|float|null.

```rust,no_run
use phper::classes::ClassEntity;

let mut foo = ClassEntity::new("Foo");
foo.add_constant("ONE", "one");
foo.add_constant("TWO", 2);
foo.add_constant("THREE", 3.0);
```

## Handle state

> The `ClassEntity` represents the class entry hold the state as generic type,
> so you can wrap the Rust struct as state in PHP class, which is the common usage
> of class in php extensions (if using C/C++ to develop PHP extension, the PHP class
> commonly wrap the C/C++ pointer).

Imagine that we encapsulate Rust's HashMap for PHP.

First, we register the class hold the state with type `HashMap`, then add the method
to insert key value pair.

```rust,no_run
use std::collections::HashMap;
use phper::classes::{ClassEntity, ClassEntry, Visibility};
use phper::functions::Argument;

let mut class =
ClassEntity::<HashMap<String, String>>::new_with_state_constructor(
    "MyHashMap", HashMap::new);

class.add_method(
    "insert",
    Visibility::Public,
    |this, arguments| {
        let key = arguments[0].expect_z_str()?.to_str()?.to_string();
        let value = arguments[1].expect_z_str()?.to_str()?.to_string();

        let state = this.as_mut_state();
        state.insert(key, value);

        Ok::<_, phper::Error>(())
    },
)
.argument(Argument::new("key"))
.argument(Argument::new("value"));
```

Equivalent to the following PHP code (hides the implementation details):

```php
<?php

class MyHashMap {

    public void insert() {
        // ...
    }

}
```

## Circular class dependencies
If you wish to register classes which depend on each other, you can retrieve the bound class (`StateClass<T>`)
from a `ClassEntity`, and use it in functions and methods:

```rust,no_run
use phper::{
    classes::{ClassEntity, StateClass, Visibility},
    modules::Module,
};

let mut module = Module::new(
    env!("CARGO_CRATE_NAME"),
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_AUTHORS"),
);

let mut a_cls = ClassEntity::new("A");
let mut b_cls = ClassEntity::new("B");
let a_bound_class = a_cls.bound_class();
let b_bound_class = b_cls.bound_class();

a_cls.add_static_method("createB", Visibility::Public, move |_| {
    let object = b_bound_class.init_object()?;
    Ok::<_, phper::Error>(object)
});
b_cls.add_static_method("createA", Visibility::Public, move |_| {
    let object = a_bound_class.init_object()?;
    Ok::<_, phper::Error>(object)
});

module.add_class(a_cls);
module.add_class(b_cls);
```

This is equivalent to the following PHP code:
```php
class A {
  public static function createB(): B {
    return new B();
  }
}
class B {
  public static function createA(): A {
    return new A();
  }
}
```
