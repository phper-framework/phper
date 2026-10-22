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

An argument is **required** and untyped unless you say otherwise. The
modifiers that change that:

| Modifier | Effect |
|---|---|
| [`optional`](phper::functions::Argument::optional) | the argument may be omitted |
| [`with_default_value`](phper::functions::Argument::with_default_value) | the default shown by reflection, and used for an argument skipped by name. Implies `optional`. On PHP 7.x, the same as `optional` |
| [`with_type_hint`](phper::functions::Argument::with_type_hint) / [`allow_null`](phper::functions::Argument::allow_null) | the declared type, and whether it accepts `null` |
| [`by_ref`](phper::functions::Argument::by_ref) | pass by reference rather than by value |

Here we define a function that accepts a nullable class (in this case, an
interface), a string with a default value, and an optional bool:

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
        Parameter #0 [ <required> ?\MyNamespace\MyInterface $a_class ]
        Parameter #1 [ <optional> string $name = 'my_default' ]
        Parameter #2 [ <optional> bool $optional_bool = <default> ]
      }
    }
```

### Declare a union type

[`ArgumentTypeHint::Union`](phper::types::ArgumentTypeHint::Union) and
[`ReturnTypeHint::Union`](phper::types::ReturnTypeHint::Union) declare a type
such as `int|string` or `Foo|false|null`. `False` is available as a member, or
on its own. [`allow_null`](phper::functions::Argument::allow_null) on a union
adds `null`, the same as a `Null` member.

```rust,no_run
use phper::{modules::Module, php_get_module, functions::{Argument, ReturnType}};
use phper::types::{ArgumentTypeHint, ReturnTypeHint};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_function("my_function", |_| -> phper::Result<bool> {
        Ok(false)
    })
    .argument(Argument::new("key").with_type_hint(ArgumentTypeHint::Union(vec![
        ArgumentTypeHint::Int,
        ArgumentTypeHint::String,
    ])))
    .argument(Argument::new("parent").with_type_hint(ArgumentTypeHint::Union(vec![
        ArgumentTypeHint::ClassEntry(String::from(r"MyNamespace\MyInterface")),
        ArgumentTypeHint::False,
        ArgumentTypeHint::Null,
    ])).with_default_value("null"))
    .return_type(ReturnType::new(ReturnTypeHint::Union(vec![
        ReturnTypeHint::String,
        ReturnTypeHint::False,
    ])));

    module
}
```

The output of `php --re` for this function would look like:

```txt
    Function [ <internal:integration> function my_function ] {

      - Parameters [2] {
        Parameter #0 [ <required> string|int $key ]
        Parameter #1 [ <optional> MyNamespace\MyInterface|false|null $parent = null ]
      }
      - Return [ string|false ]
    }
```

Version notes:

- Union types and `false` need PHP 8.0. On PHP 7.x the argument or return is
  declared untyped.
- A union of two or more classes needs PHP 8.0.2. Earlier 8.0 releases cannot
  split the class names.
- From PHP 8.2, `iterable` in a union is shown as `Traversable|array`, as it
  is in userland.

A union that PHP would reject in userland panics when the module registers
it, on every PHP version:

- fewer than two members, which the `union_type` rule in
  `Zend/zend_language_parser.y` cannot express
- a nested union
- `mixed`, `void` or `never` as a member
- a duplicate member, or a redundant pair such as `bool|false`, `object|Foo`,
  `iterable|array` or `iterable|Traversable`

The last two are the checks `zend_compile_typename` in `Zend/zend_compile.c`
applies to userland code. PHP does not apply any of them to internal
functions.

### Default values reach the handler only through named arguments

A default value works with any type hint, or none. It is recorded on PHP 8.0
and later. Older versions ignore it, but the argument is still optional.

In userland, PHP binds every default to its parameter, even when the caller
leaves it off:

```php
function my_function(?\MyNamespace\MyInterface $a_class, string $name = 'my_default', bool $optional_bool = false) {
    var_dump(func_get_args(), $name, $optional_bool);
}

my_function(null);
// func_get_args() is [null]
// $name is 'my_default'
// $optional_bool is false
```

`func_get_args()` holds only the passed arguments. As the
[PHP manual](https://www.php.net/manual/en/function.func-get-args.php) puts
it, it "does not account for default (non-passed) arguments".

A phper handler has no parameter variables. Its `arguments` slice holds what
`func_get_args()` would return, and nothing more:

```php
// All three passed. The handler gets null, 'x', true.
my_function(null, 'x', true);

// $name skipped by name. PHP fills the gap from the declared default, so the
// handler gets null, 'my_default', true.
my_function(null, optional_bool: true);

// Only $a_class passed. The handler gets null, and nothing for $name or
// $optional_bool.
my_function(null);
```

PHP's own internal functions work the same way. `str_pad` declares
`$pad_string = " "`, but the default it uses is a C local variable. The
declared default is for reflection and named-argument gaps.

Read optional arguments with `arguments.get(n)`. Indexing past the end of the
slice panics, which aborts the PHP process. What to do with a missing argument
is up to the handler. This one falls back to the declared defaults, as a
userland function would:

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

    module.add_function("my_function", |arguments| -> phper::Result<()> {
        let name = match arguments.get(1) {
            Some(name) => name.expect_z_str()?.to_str()?,
            None => "my_default",
        };
        let optional_bool = match arguments.get(2) {
            Some(optional_bool) => optional_bool.expect_bool()?,
            None => false,
        };
        echo!("{} {}\n", name, optional_bool);
        Ok(())
    })
    .argument(Argument::new("a_class").with_type_hint(ArgumentTypeHint::ClassEntry(String::from(r"\MyNamespace\MyInterface"))).allow_null())
    .argument(Argument::new("name").with_type_hint(ArgumentTypeHint::String).with_default_value("'my_default'"))
    .argument(Argument::new("optional_bool").with_type_hint(ArgumentTypeHint::Bool).optional());

    module
}
```

### Optional arguments must come last

Note the ordering in that example: the two optional arguments come after the
required one. That is not a style choice.

PHP derives optionality from *position*, not from a per-argument flag — it
records the index of the last required parameter and treats everything before
it as required, however that argument was declared. So an `optional` argument
placed *before* a required one is promoted back to required, and its default
value is never used. This is the same rule the PHP compiler applies to
userland functions, where writing it raises a deprecation notice as of PHP
8.0.

```rust,no_run
use phper::{modules::Module, php_get_module, functions::Argument};

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
    .argument(Argument::new("a"))
    .argument(Argument::new("b").optional())
    .argument(Argument::new("c"));

    module
}
```

Despite `b` being declared optional, all three parameters are required, and
calling with fewer than three raises `ArgumentCountError`:

```txt
    Function [ <internal:integration> function my_function ] {

      - Parameters [3] {
        Parameter #0 [ <required> $a ]
        Parameter #1 [ <required> $b ]
        Parameter #2 [ <required> $c ]
      }
    }
```

Declare optional arguments last, so the signature means what it reads as.
