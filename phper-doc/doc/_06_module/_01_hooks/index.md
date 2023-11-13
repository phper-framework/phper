# Hooks

> PHP is a complex piece of machinery, and its lifecycle should be understood
> by anyone who wants to grasp how PHP operates.
>
> Refer: <https://www.phpinternalsbook.com/php7/extensions_design/php_lifecycle.html>

PHP provides many hooks in lifecycle for extension to override.

There are `MINIT`, `MSHUTDOWN`, `RINIT`, `RSHUTDOWN`, `GINIT`, `RSHUTDOWN`.

Correspondingly, `PHPER` sets these hooks to complete some internal operations,
such as registering extension information, functions, classes, constants, etc.
However, it also exposes these hooks to users for overwriting.

 The following presents the corresponding relationships between PHP hooks and `Module`
 methods:

| PHP hooks | `Module` method                                          |
| --------- | -------------------------------------------------------- |
| MINIT     | [on_module_init](phper::modules::Module::on_module_init) |
| MSHUTDOWN     | [on_module_shutdown](phper::modules::Module::on_module_shutdown) |
| RINIT     | [on_request_init](phper::modules::Module::on_request_init) |
| RSHUTDOWN     | [on_request_shutdown](phper::modules::Module::on_request_shutdown) |


```rust,no_run
use phper::{modules::Module, php_get_module};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.on_module_init(|| {
        // Do somethings in `MINIT` stage.
    });

    module
}
```
