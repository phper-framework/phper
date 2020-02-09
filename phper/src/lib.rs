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

pub extern crate phper_alloc as alloc;
extern crate phper_macros;
pub extern crate phper_sys as sys;

mod macros;

pub use phper_macros::*;
pub use phper_sys::c_str_ptr;
use sys::{zend_function_entry, zend_ini_entry_def, zend_module_entry};
use thiserror::Error;

mod function;
mod module;
mod types;

pub use crate::function::*;
pub use crate::module::*;
pub use crate::types::*;

//pub type IniEntries = Vec<zend_ini_entry_def>;
//
//pub type StaticZendModuleEntry = NotThreadSafe<*const zend_module_entry>;
//
//pub type StaticZendFunctionEntry = NotThreadSafe<*const zend_function_entry>;
//
//pub struct NotThreadSafe<T>(pub T);
//
//unsafe impl<T> Sync for NotThreadSafe<T> {}

type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    ModuleBuild(ModuleBuildError),
}

