# Zval

> Refer to: <https://www.phpinternalsbook.com/php7/zvals/basic_structure.html#the-zval-struct>
>
> A zval (short for “Zend value”) represents an arbitrary PHP value.
> As such it is likely the most important structure in all of PHP and
> you’ll be working with it a lot.

The [`phper::values::ZVal`] is the wrapper of php zval.

## Actual type of ZVal

PHP is a dynamically typed language, zval can represent multiple types,
but there is only one type at a time, you can use
[`phper::values::ZVal::get_type_info`] to get the actual type.

## Convert Rust type to ZVal

The [`phper::values::ZVal`] implements a lot of [`std::convert::From`] for the
conversion.

Here is the mapping relationship of Rust type and base PHP type.

| Trait           | Rust type                   | PHP type |
| --------------- | --------------------------- | -------- |
| `From<()>`      | `()`                        | null     |
| `From<bool>`    | `bool`                      | bool     |
| `From<i64>`     | `i64`                       | long     |
| `From<f64>`     | `f64`                       | double   |
| `From<&str>`    | `&str`                      | string   |
| `From<&CStr>`   | `&CStr`                     | string   |
| `From<&[u8]>`   | `&[u8]`                     | string   |
| `From<Vec<u8>>` | `Vec<u8>`                   | string   |
| `From<ZString>` | [`phper::strings::ZString`] | string   |
| `From<ZArray>`  | [`phper::arrays::ZArray`]   | array    |
| `From<ZObject>` | [`phper::objects::ZObject`] | object   |

Otherwise, there are also composite types that implement `From`.

- `From<Option<T>>`: if Some(T), T will be converted to PHP type like `From<T>`,
  or `None` wll be converted to `null`.

- `From<Result<T, E>>`: if Ok(T), T will be converted to PHP type like `From<T>`,
  or `Err(e)` will throw an Exception by calling `zend_throw_exception`.

### Example

```rust,no_run
use phper::values::ZVal;

assert!(ZVal::from(()).get_type_info().is_null());
assert!(ZVal::from(100i64).get_type_info().is_long());
```

## Convert ZVal to Rust type

Now you can use `as_*` or `expect_*` methods to convert ZVal to Rust types.

- The `as_*` returns `Option<T>`.

- The `expect_*` returns `phper::Result<T>`, if convert failed,
  [phper::errors::ExpectTypeError] will be returned, with the message:
  `type error: must be of type {expect_type}, {actual_type} given")`.


### Example

```rust,no_run
use phper::echo;
use phper::values::ZVal;

fn say_hello(arguments: &mut [ZVal]) -> phper::Result<()> {
    // Get the first argument, expect the type `ZStr`, and convert to Rust utf-8
    // str.
    let name = arguments[0].expect_z_str()?.to_str()?;

    // Macro which do php internal `echo`.
    echo!("Hello, {}!\n", name);

    Ok(())
}
```

## Value copy & reference counting copy

The [`phper::values::ZVal`] both implements [`std::clone::Clone`] and
[`phper::alloc::RefClone`].

- [`std::clone::Clone`]: represent value copy (Type ZObject is excluded because it is always passed by reference).

- [`phper::alloc::RefClone`]: represent reference counting  copy (Type (), bool,
  i64, f64 is excluded because they are not reference counting types).

## PHP internal cast

> Refer to: <https://www.phpinternalsbook.com/php7/zvals/casts_and_operations.html#casts>

PHP is a weakly typed language, supports type cast internally.

The zend engine provides `convert_to_*` functions to do the type cast, and
`ZVal` wraps them directly.

## Callable

The [`phper::values::ZVal`] can be call via [`phper::values::ZVal::call`], make
sure the actual type is callable (string or array or closure).

```rust,no_run
use phper::values::ZVal;
use phper::arrays::ZArray;

let mut arr = ZArray::new();
arr.insert("a", ZVal::from(1));
arr.insert("b", ZVal::from(2));
let ret = ZVal::from("json_encode").call(&mut [ZVal::from(arr)]).unwrap();
assert_eq!(ret.expect_z_str().unwrap().to_str(), Ok(r#"{"a":1,"b":2}"#));
```
