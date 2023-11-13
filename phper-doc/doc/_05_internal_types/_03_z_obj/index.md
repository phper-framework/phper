# Z Obj

The [`&ZObj`](phper::objects::ZObj) and [`ZObject`](phper::objects::ZObject) are
the wrappers for [`zend_object`](phper::sys::zend_object).

You can do OOP operation using `ZObj` or `ZObject`, like getting and setting properties,
calling methods, etc.

```rust,no_run
use phper::classes::ClassEntry;
use phper::objects::ZObject;
use phper::errors::exception_class;

let mut e: ZObject = exception_class().new_object([]).unwrap();
e.set_property("code", 403);
e.set_property("message", "oh no");
let _message = e.call("getMessage", []).unwrap();
```

`ZObj` implements `ToRefOwned` to upgrade to `ZObject`, duplicate the object via increment refcount.

`ZObject` implements `RefClone`, same as `ZObj::to_owned`.

```rust,no_run
use phper::sys;
use phper::objects::ZObj;
use phper::alloc::ToRefOwned;

extern "C" {
    fn something() -> *mut sys::zend_object;
}

let o = unsafe { ZObj::from_mut_ptr(something()) };

// By refcount increment.
let _o = o.to_ref_owned();
```

Note that neither `ZObj` nor `ZObject` implements `Send` and `Sync`, because PHP
is single-threaded.
