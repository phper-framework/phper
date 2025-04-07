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

## Using EnumCase for Direct Access

When you call `add_case()` on an enum entity, it returns an `EnumCase` instance that you can use for direct access to that case later. This is more efficient than looking up the enum case by name each time you need it.

Here's an example showing how to use EnumCase within static methods of the enum:

```rust,no_run
use phper::{
    modules::Module, 
    php_get_module, 
    enums::EnumEntity, 
    classes::Visibility,
    alloc::ToRefOwned,
};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Create a pure enum
    let mut pure_enum_entity = EnumEntity::new("PureEnum");
    
    // Store references to enum cases when adding them
    let one_case = pure_enum_entity.add_case("ONE", ());
    let _two_case = pure_enum_entity.add_case("TWO", ());
    
    // Add static method that returns the ONE case
    pure_enum_entity.add_static_method("getOneCase", Visibility::Public, {
        // Clone the EnumCase because it will be moved into the closure
        move |_| {
            // Get the case object directly from EnumCase
            let one_obj = one_case.clone().get_mut_case();
            let result = one_obj.to_ref_owned();
            phper::ok(result)
        }
    });
    
    // Register the enum to the module
    module.add_enum(pure_enum_entity);

    // Create an int-backed enum
    let mut int_enum_entity = EnumEntity::<i64>::new("IntEnum");
    
    // Store reference to LOW case
    let low_case = int_enum_entity.add_case("LOW", 10);
    let _high_case = int_enum_entity.add_case("HIGH", 100);
    
    // Add static method that returns the LOW case
    int_enum_entity.add_static_method("getLowCase", Visibility::Public, {
        move |_| {
            let low_obj = low_case.clone().get_mut_case();
            let result = low_obj.to_ref_owned();
            phper::ok(result)
        }
    });
    
    // Register the enum to the module
    module.add_enum(int_enum_entity);

    module
}
```

This creates PHP enums with static methods that can access specific cases:

```php
enum PureEnum {
    case ONE;
    case TWO;
    
    public static function getOneCase(): self {
        return self::ONE;
    }
}

enum IntEnum: int {
    case LOW = 10;
    case HIGH = 100;
    
    public static function getLowCase(): self {
        return self::LOW;
    }
}
```

## Using Enum::from_name

If you don't have direct access to the EnumCase, you can use `Enum::from_name()` to get an enum by its name, and then use `get_case()` or `get_mut_case()` to access specific cases:

```rust,no_run
use phper::{
    enums::Enum,
    enums::EnumEntity,
    classes::Visibility,
    alloc::ToRefOwned,
};

fn create_enum_with_dynamic_lookup() -> EnumEntity {
    let mut enum_entity = EnumEntity::new("DynamicEnum");
    enum_entity.add_case("ONE", ());
    enum_entity.add_case("TWO", ());
    
    // Add a static method that looks up cases dynamically
    enum_entity.add_static_method("getCaseByName", Visibility::Public, |args| {
        // Get case name from parameters
        let case_name = args[0].expect_z_str()?.to_string_lossy();
        
        // Use Enum::from_name to get the enum
        let mut enum_obj = Enum::from_name("DynamicEnum");
        
        // Try to get the requested case
        let case = unsafe { enum_obj.get_mut_case(&case_name)? };
        let result = case.to_ref_owned();
        
        phper::ok(result)
    });
    
    enum_entity
}
```

> **Important**: The `get_case()` and `get_mut_case()` methods on `Enum` are marked as unsafe because they can cause segmentation faults if the case doesn't exist.

## Bound Enum

You can use the `bound_enum()` method to get a reference to the enum that can be used in methods or functions:

```rust,no_run
use phper::{enums::EnumEntity, classes::Visibility, alloc::ToRefOwned};

pub fn make_status_enum() -> EnumEntity {
    let mut enum_entity = EnumEntity::new("Status");
    enum_entity.add_case("Active", ());
    enum_entity.add_case("Inactive", ());
    
    // Get a reference to the enum that will be created
    let status_enum = enum_entity.bound_enum();
    
    // Add a static method that uses the bound enum
    enum_entity.add_static_method("getActiveCase", Visibility::Public, move |_| {
        // Use the bound enum to get the Active case
        let active_case = unsafe { status_enum.clone().get_mut_case("Active")? };
        phper::ok(active_case.to_ref_owned())
    });
    
    enum_entity
}
```

## Complete Example

Here's a comprehensive example using both pure and backed enums with static methods:

```rust,no_run
use phper::{
    modules::Module, 
    php_get_module, 
    enums::EnumEntity, 
    classes::Visibility,
    alloc::ToRefOwned,
};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Pure enum
    let mut status = EnumEntity::new("Status");
    let pending_case = status.add_case("PENDING", ());
    let active_case = status.add_case("ACTIVE", ());
    let inactive_case = status.add_case("INACTIVE", ());
    
    // Add constants
    status.add_constant("VERSION", "1.0.0");
    
    // Add static method that returns the active state
    status.add_static_method("getActiveStatus", Visibility::Public, {
        move |_| {
            let obj = active_case.clone().get_mut_case();
            phper::ok(obj.to_ref_owned())
        }
    });
    
    // Add static method that returns status description
    status.add_static_method("getDescription", Visibility::Public, |_| {
        phper::ok("Status enumeration")
    });
    
    // Integer-backed enum
    let mut level = EnumEntity::<i64>::new("Level");
    let low_case = level.add_case("LOW", 1);
    let medium_case = level.add_case("MEDIUM", 5);
    let high_case = level.add_case("HIGH", 10);
    
    // Add static method that returns level by value
    level.add_static_method("getLevelByValue", Visibility::Public, {
        move |args| {
            let value: i64 = args[0].expect_long()?;
            let case_obj = match value {
                v if v < 3 => low_case.clone().get_mut_case(),
                v if v < 8 => medium_case.clone().get_mut_case(),
                _ => high_case.clone().get_mut_case(),
            };
            phper::ok(case_obj.to_ref_owned())
        }
    });
    
    // Register enums to the module
    module.add_enum(status);
    module.add_enum(level);

    module
}
```

> **Note**: PHP enums require PHP 8.1 or higher. Make sure your extension sets the correct PHP version requirements.
