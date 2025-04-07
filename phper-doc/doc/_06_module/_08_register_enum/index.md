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

## Accessing Enums from Rust Code

After registering an enum, you might need to access it from Rust code. `PHPER` provides two ways to do this:

1. Using the returned `Enum` instance when registering the enum
2. Using `Enum::from_name` to look up an enum by name

### Using the Returned `Enum` Instance

When you register an enum using `module.add_enum()`, it returns an `Enum` instance that you can save and use later.

```rust,no_run
use phper::{modules::Module, php_get_module, enums::{EnumEntity, Enum}};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Create and register the enum
    let mut status_entity = EnumEntity::new("Status");
    status_entity.add_case("ACTIVE", ());
    status_entity.add_case("INACTIVE", ());
    
    // Save the returned Enum instance
    let status_enum: Enum = module.add_enum(status_entity);
    
    // Use the saved enum instance in a function
    module.add_function("get_active_status", move |_| {
        // Get the ACTIVE case from the enum
        let active_case = status_enum.get_case("ACTIVE")?;
        Ok::<_, phper::Error>(active_case)
    });

    module
}
```

### Using `Enum::from_name`

If you don't have the original `Enum` instance, you can use `Enum::from_name` to look up an enum by its name.

```rust,no_run
use phper::{enums::Enum, modules::Module, php_get_module};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );
    
    // Register functions that use enums
    module.add_function("get_status_case", |args| {
        // Look up the Status enum by name
        let status_enum = Enum::from_name("Status");
        
        // Get the case name from the function arguments
        let case_name = args[0].as_z_str()?.to_str()?;
        
        // Get the requested enum case
        let case = status_enum.get_case(case_name)?;
        
        Ok::<_, phper::Error>(case)
    });

    module
}
```

## Getting Enum Cases

Once you have an `Enum` instance, you can use the `get_case` method to access specific enum cases:

```rust,no_run
// Get a case from a pure enum
let status_enum = Enum::from_name("Status");
let active_case = status_enum.get_case("ACTIVE")?;

// Get a case from a backed enum
let level_enum = Enum::from_name("Level");
let high_level = level_enum.get_case("HIGH")?;
```

If you need to modify an enum case's properties, you can use `get_case_mut` to get a mutable reference:

```rust,no_run
// Get a mutable reference to an enum case
let mut status_enum = Enum::from_name("Status");
let mut active_case = status_enum.get_case_mut("ACTIVE")?;

// Now you can modify the case object if needed
```

If the specified case doesn't exist in the enum, both `get_case` and `get_case_mut` will return an `EnumCaseNotFoundError`.

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
