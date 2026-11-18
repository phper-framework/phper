# Z Str

> A string is series of characters, where a character is the same as a byte.
>
> Refer: <https://www.php.net/manual/en/language.types.string.php>

The [`&ZStr`](https://github.com/phper-framework/phper/blob/master/phper/src/strings.rs) and [`ZString`](https://github.com/phper-framework/phper/blob/master/phper/src/strings.rs) are
wrappers for [`zend_string`](https://github.com/php/php-src/blob/master/Zend/zend_types.h).

`ZStr` can be converted to `&[u8]`, `&CStr` and `&str`.

`ZString` can be constructed from `impl AsRef<[u8]>` and has pair of `from_raw()`
and `into_raw()`, like in [`Box`].

```rust,no_run
use phper::strings::ZString;

let s = ZString::new("Hello world!");

// Will leak memory.
let ptr = ZString::into_raw(s);

// retake pointer.
let ss = unsafe { ZString::from_raw(ptr) };

// `ZString` implements `PartialEq`.
assert_eq!(ss, "Hello world!");
```

`ZString` can be dereferenced to `ZStr`.

```rust,no_run
use phper::strings::ZString;

let s = ZString::new("Hello world!");

// `to_str` is the method of `ZStr`.
assert_eq!(s.to_str(), Ok("Hello world!"));
```

`ZStr` implements `ToOwned`. It can upgrade to `ZString` by value copying.

Because `zend_string` is reference counting type, so `ZStr` also implements
[`ToRefOwned`](https://github.com/phper-framework/phper/blob/master/phper-alloc/src/lib.rs) (just like
[`RefClone`](https://github.com/phper-framework/phper/blob/master/phper-alloc/src/lib.rs) for [`ZVal`](https://github.com/phper-framework/phper/blob/master/phper/src/values.rs)), can
upgrade to `ZString` by refcount increment.

```rust,no_run
use phper::sys;
use phper::strings::ZStr;
use phper::alloc::ToRefOwned;

unsafe extern "C" {
    fn something() -> *mut sys::zend_string;
}

let s = unsafe { ZStr::from_mut_ptr(something()) };

// By value copying.
let _s = s.to_owned();

// By refcount increment.
let _s = s.to_ref_owned();
```

Note that neither `ZStr` nor `ZString` implements `Send` and `Sync`, because PHP
is single-threaded.
