# Register Enums

> PHP 8.1 and above introduced the Enum functionality. `PHPER` allowing you to create PHP enums using Rust code.

In `PHPER`, you can use the [`add_enum`](phper::modules::Module::add_enum) method to register enums.
According to PHP's enum specification, `PHPER` supports three types of enums:

1. Pure enums (without values)
2. Integer-backed enums
3. String-backed enums

## Creating Pure Enums

Pure enums are the simplest type of enum, having only member names without associated values. Use `EnumEntity<()>` to create a pure enum (or simply use `EnumEntity::new()` since `()` is the default type parameter).

```rust,no_run
use phper::{modules::Module, php_get_module, enums::EnumEntity, classes::Visibility};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Create a pure enum
    let mut status = EnumEntity::new("Status");
    
    // Add enum cases (without values)
    status.add_case("PENDING", ());
    status.add_case("ACTIVE", ());
    status.add_case("INACTIVE", ());
    
    // Register the enum to the module
    module.add_enum(status);

    module
}
```

This is equivalent to the following PHP code:

```php
enum Status {
    case PENDING;
    case ACTIVE;
    case INACTIVE;
}
```

## Creating Integer-Backed Enums

Integer-backed enums associate each enum member with an integer value. Use `EnumEntity<i64>` to create an integer-backed enum.

```rust,no_run
use phper::{modules::Module, php_get_module, enums::EnumEntity};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Create an integer-backed enum
    let mut level = EnumEntity::<i64>::new("Level");
    
    // Add enum cases with their associated integer values
    level.add_case("LOW", 1);
    level.add_case("MEDIUM", 5);
    level.add_case("HIGH", 10);
    
    // Register the enum to the module
    module.add_enum(level);

    module
}
```

This is equivalent to the following PHP code:

```php
enum Level: int {
    case LOW = 1;
    case MEDIUM = 5;
    case HIGH = 10;
}
```

## Creating String-Backed Enums

String-backed enums associate each enum member with a string value. Use `EnumEntity<String>` to create a string-backed enum.

```rust,no_run
use phper::{modules::Module, php_get_module, enums::EnumEntity};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Create a string-backed enum
    let mut color = EnumEntity::<String>::new("Color");
    
    // Add enum cases with their associated string values
    color.add_case("RED", "FF0000".to_string());
    color.add_case("GREEN", "00FF00".to_string());
    color.add_case("BLUE", "0000FF".to_string());
    
    // Register the enum to the module
    module.add_enum(color);

    module
}
```

This is equivalent to the following PHP code:

```php
enum Color: string {
    case RED = "FF0000";
    case GREEN = "00FF00";
    case BLUE = "0000FF";
}
```

## Adding Constants

Enums can contain constants. Use the `add_constant` method to add constants to an enum.

```rust,no_run
use phper::{modules::Module, php_get_module, enums::EnumEntity};

let mut status = EnumEntity::new("Status");

// Add enum cases
status.add_case("PENDING", ());
status.add_case("ACTIVE", ());

// Add constants
status.add_constant("VERSION", "1.0.0");
status.add_constant("MAX_ATTEMPTS", 3);
```

This is equivalent to the following PHP code:

```php
enum Status {
    case PENDING;
    case ACTIVE;
    
    public const VERSION = "1.0.0";
    public const MAX_ATTEMPTS = 3;
}
```

## Adding Static Methods

You can add static methods to enums. Use the `add_static_method` method to add a static method to an enum.

```rust,no_run
use phper::{modules::Module, php_get_module, enums::EnumEntity, classes::Visibility};
use std::convert::Infallible;

let mut status = EnumEntity::new("Status");

// Add enum cases
status.add_case("PENDING", ());
status.add_case("ACTIVE", ());

// Add static method
status.add_static_method("getDescription", Visibility::Public, |_| {
    Ok::<_, Infallible>("Status enumeration for tracking item states")
});
```

This is equivalent to the following PHP code:

```php
enum Status {
    case PENDING;
    case ACTIVE;
    
    public static function getDescription(): string {
        return "Status enumeration for tracking item states";
    }
}
```

## Implementing Interfaces

You can make enums implement interfaces. Use the `implements` method to make an enum implement a specific interface.

```rust,no_run
use phper::{modules::Module, php_get_module, enums::EnumEntity, classes::Interface};

let mut color = EnumEntity::<String>::new("Color");

// Add enum cases
color.add_case("RED", "FF0000".to_string());
color.add_case("GREEN", "00FF00".to_string());

// Implement interface
color.implements(Interface::from_name("JsonSerializable"));

// Note: You need to add necessary methods to satisfy interface requirements
// For example, JsonSerializable interface requires the implementation of jsonSerialize method
```

## Using Built-in Enum Methods

PHP enums come with some built-in methods. Pure enums (`UnitEnum`) have the `cases()` method, while backed enums (`BackedEnum`) additionally have the `from()` and `tryFrom()` methods.

```php
// Examples of using built-in methods in PHP
$allCases = Color::cases(); // Returns an array of all enum cases

// Only available for backed enums
$colorFromValue = Color::from("FF0000"); // Returns RED
$colorOrNull = Color::tryFrom("INVALID"); // Returns null (when the value doesn't exist)
```

## Complete Example

Here's a comprehensive example using both pure and backed enums:

```rust,no_run
use phper::{
    modules::Module, 
    php_get_module, 
    enums::EnumEntity, 
    classes::Visibility
};
use std::convert::Infallible;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Pure enum
    let mut status = EnumEntity::new("Status");
    status.add_case("PENDING", ());
    status.add_case("ACTIVE", ());
    status.add_case("INACTIVE", ());
    status.add_constant("VERSION", "1.0.0");
    status.add_static_method("getDescription", Visibility::Public, |_| {
        Ok::<_, Infallible>("Status enumeration")
    });
    
    // Integer-backed enum
    let mut level = EnumEntity::<i64>::new("Level");
    level.add_case("LOW", 1);
    level.add_case("MEDIUM", 5);
    level.add_case("HIGH", 10);
    
    // Register enums to the module
    module.add_enum(status);
    module.add_enum(level);

    module
}
```

> **Note**: PHP enums require PHP 8.1 or higher. Make sure your extension sets the correct PHP version requirements.
