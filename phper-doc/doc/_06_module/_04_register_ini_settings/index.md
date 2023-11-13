# Register ini settings

In `PHPER`, you can use [`add_ini`](phper::modules::Module::add_ini) to 
register ini settings.

```rust,no_run
use phper::{modules::Module, php_get_module, ini::Policy};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_ini("demo.enable", false, Policy::All);
    module.add_ini("demo.foo", 100, Policy::All);

    module
}
```

About the policy of setting, you can refer to
<https://www.php.net/manual/en/configuration.changes.modes.php>.

## Configure ini settings

The user can configure the ini settings in the `php.ini`. If not configured, the
configuration item will use the default value.

```ini
demo.enable = On
```

You can show the ini settings by `php --ri <EXTENSION_NAME>`.

```txt
demo

version => 0.0.0
authors => PHPER Framework Team:jmjoy <jmjoy@apache.org>

Directive => Local Value => Master Value
demo.enable => 1 => 1
demo.num => 100 => 100
```

## Get ini settings

After the ini settings registered, you can get it by
[`ini_get`](phper::ini::ini_get).

```rust,no_run
use phper::ini::ini_get;

let _foo = ini_get::<bool>("demo.enable");
let _bar = ini_get::<i64>("demo.foo");
```
