# Internal types

## DST & Owned Type

In Rust, there are many types that appear in pairs, like [str](str) / [String](String),
[OsStr](std::ffi::OsStr) / [OsString](std::ffi::OsString),
[CStr](std::ffi::CStr) / [CString](std::ffi::CString).

For example:

- [str](str): Dynamically sized type, implements `!Sized`, usually used with reference
  notation, as `&str`.
- [String](String): Ownership type, encapsulates a pointer to a heap memory allocation.

PHPER follows this design, there are the following types:

- [ZStr](phper::strings::ZStr) / [ZString](phper::strings::ZString)
- [ZArr](phper::arrays::ZArr) / [ZArray](phper::arrays::ZArray)
- [ZObj](phper::objects::ZObj) / [ZObject](phper::objects::ZObject)

## Mapping relationship

Here is the mapping relationship of Rust type and base PHP type.

| Rust type        | PHP type |
| ---------------- | -------- |
| `()`             | null     |
| `bool`           | bool     |
| `i64`            | long     |
| `f64`            | double   |
| `ZStr / ZString` | string   |
| `ZArr / ZArray`  | array    |
| `ZObj / ZObject` | object   |
| `ZRes`           | resource |

*Why is there no ZResource? Because Resource is a relatively old type, it*
*is generally replaced by Class now, and the role of ZRes is only compatible*
*with old extension resources.*
