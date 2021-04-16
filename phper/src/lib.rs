#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]

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

pub mod arrays;
pub mod classes;
pub mod cmd;
mod errors;
pub mod functions;
pub mod ini;
pub mod logs;
pub mod modules;
pub mod strings;
mod utils;
pub mod values;

pub use crate::errors::*;
pub use phper_alloc as alloc;
pub use phper_macros::*;
pub use phper_sys as sys;
