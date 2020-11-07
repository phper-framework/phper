#![feature(min_const_generics)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_transmute)]

/*!
A library that allows us to write PHP extensions using pure Rust and using safe Rust whenever possible.

***Now the peojct is still under development.***

# Usage

First you have to install `cargo-generate`:

```bash
cargo install cargo-generate
```

Then create a PHP extension project from the [template](https://github.com/jmjoy/phper-ext-skel.git):

```bash
cargo generate --git https://github.com/jmjoy/phper-ext-skel.git
```

# Notice

Now the library don't support `ZTS`, the template is using `thread_local!` instead.

Version `0.1.x` will be a preview version.
*/

pub mod zend;
mod error;
mod utils;

pub use phper_alloc as alloc;
pub use phper_sys as sys;
pub use phper_macros::*;
pub use crate::error::*;
