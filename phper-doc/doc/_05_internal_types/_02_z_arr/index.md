# Z Arr

> An array in PHP is actually an ordered map. A map is a type that associates
> values to keys. This type is optimized for several different uses; it can be
> treated as an array, list (vector), hash table (an implementation of a map),
> dictionary, collection, stack, queue, and probably more. As array values can
> be other arrays, trees and multidimensional arrays are also possible.
>
> Refer: <https://www.php.net/manual/en/language.types.array.php>

*In fact, I don't agree with PHP's practice of mixing list and map. I prefer*
*python to separate list and dictionary as two types. For example, when*
*serializing into json, the serialization function has to judge whether the key*
*of the array starts from 0 and increment by 1 to confirm whether the array is*
*a list. I think it is a waste of performance.*

The [`&ZArr`](phper::arrays::ZArr) and [`ZArray`](phper::arrays::ZArray) are
the wrappers for [`zend_array`](phper::sys::zend_array) (same as `Hashtable`).

[`&ZArr`](phper::arrays::ZArr) acts like [`HashMap`](std::collections::HashMap),
also has api `insert()`, `get()`, `remove()`, but it's key type is
[`Key`](phper::arrays::Key) and value type is [`ZVal`](phper::values::ZVal).

Notice that phper prefers to use [`Symtables`](https://www.phpinternalsbook.com/php5/hashtables/array_api.html#symtables) api `zend_symtable_*`,
so `get(42)` and `get("42")` should be considered the same.

`ZArray` can be dereferenced to `ZArr`.

```rust,no_run
use phper::arrays::{ZArray, InsertKey};
use phper::values::ZVal;

let mut arr = ZArray::new();

arr.insert(InsertKey::NextIndex, ZVal::default());
arr.insert(10, ZVal::from(100));
arr.insert("foo", ZVal::from("bar"));

let _i = arr.get("10");

arr.remove("foo");
```

`ZArr` can be iterated by `iter()`.

```rust,no_run
use phper::arrays::ZArray;
use phper::values::ZVal;

let arr = ZArray::new();

for (k, v) in arr.iter() {
}
```

`ZArr` implements `ToOwned` and it can upgrade to `ZArray` by value copying via
`zend_array_dup`.

Because `zend_array` is reference counting type, `ZArr` also implements
[`ToRefOwned`](phper::alloc::ToRefOwned) (similar to
[`RefClone`](phper::alloc::RefClone) for [`ZVal`](phper::values::ZVal)), allowing an
upgrade to `ZArray` by incrementing the refcount.

```rust,no_run
use phper::sys;
use phper::arrays::ZArr;
use phper::alloc::ToRefOwned;

extern "C" {
    fn something() -> *mut sys::zend_array;
}

let arr = unsafe { ZArr::from_mut_ptr(something()) };

// By value copy.
let _arr = arr.to_owned(); 

// By refcount increment.
let _arr = arr.to_ref_owned();
```

Note that neither `ZArr` nor `ZArray` implements `Send` and `Sync`, because PHP
is single-threaded.
