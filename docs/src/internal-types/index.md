# Internal types

## DST & Owned Type

In Rust, there are many types that appear in pairs, like [str](https://doc.rust-lang.org/std/primitive.str.html) / [String](https://doc.rust-lang.org/std/string/struct.String.html),
[OsStr](https://doc.rust-lang.org/std/ffi/struct.OsStr.html) / [OsString](https://doc.rust-lang.org/std/ffi/struct.OsString.html),
[CStr](https://doc.rust-lang.org/std/ffi/struct.CStr.html) / [CString](https://doc.rust-lang.org/std/ffi/struct.CString.html).

For example:

- [str](https://doc.rust-lang.org/std/primitive.str.html): Dynamically sized type, implements `!Sized`, usually used with reference
  notation, as `&str`.
- [String](https://doc.rust-lang.org/std/string/struct.String.html): Ownership type, encapsulates a pointer to a heap memory allocation.

PHPER follows this design, there are the following types:

- [ZStr](https://github.com/phper-framework/phper/blob/master/phper/src/strings.rs) / [ZString](https://github.com/phper-framework/phper/blob/master/phper/src/strings.rs)
- [ZArr](https://github.com/phper-framework/phper/blob/master/phper/src/arrays.rs) / [ZArray](https://github.com/phper-framework/phper/blob/master/phper/src/arrays.rs)
- [ZObj](https://github.com/phper-framework/phper/blob/master/phper/src/objects.rs) / [ZObject](https://github.com/phper-framework/phper/blob/master/phper/src/objects.rs)

> It seems that there is no need to separate into two types, but `ZStr`,
> `ZArr`, and `ZObj` are all transparent types. Therefore, the fields of the struct
> cannot be changed. Separating an ownership struct `ZString`, `ZArray`, and `ZObject`,
> and then we can change fields of the struct in the future.

## Mapping relationship

Here is the mapping of relationships between Rust types and base PHP types.

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
