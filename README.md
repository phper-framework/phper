# PHPER (PHP Enjoy Rust)

[![CI](https://github.com/phper-framework/phper/actions/workflows/ci.yml/badge.svg)](https://github.com/phper-framework/phper/actions/workflows/ci.yml)
[![Crates](https://img.shields.io/crates/v/phper)](https://crates.io/crates/phper)
[![Docs](https://img.shields.io/docsrs/phper)](https://docs.rs/phper)
[![Lines](https://img.shields.io/tokei/lines/github/phper-framework/phper)](https://github.com/phper-framework/phper)
[![License](https://img.shields.io/crates/l/phper)](https://github.com/phper-framework/phper/blob/master/LICENSE)

## Rust ❤️ PHP

The framework that allows us to write PHP extensions using pure and safe Rust whenever possible.

## Requirement

### Necessary

- **rust** 1.56 or later
- **libclang** 9.0 or later
- **php** 8.0 or later

### Tested Support

- **OS**
  - [x] linux
  - [x] macos
  - [ ] windows
- **PHP**
  - **version**
    - [x] 8.0
    - [x] 8.1
    - [ ] 8.2
  - **mode**
    - [x] nts
    - [ ] zts
  - **sapi**
    - [x] cli
    - [x] fpm
  - **debug**
    - [x] disable
    - [ ] enable

## Usage

1. Make sure `libclang` and `php` is installed.

   ```bash
   # If you are using debian like linux system:
   sudo apt install llvm-10-dev libclang-10-dev php-cli
   ```

1. Create you cargo project, suppose your application is called myapp.

   ```bash
   cargo new myapp
   ```

1. Add the dependencies and metadata to you Cargo project.

   ```toml
   [lib]
   crate-type = ["cdylib"]
 
   [dependencies]
   phper = "<LATEST VERSION>"
   ```

1. Add these code to `main.rs`.

   ```rust,no_run
   use phper::cmd::make;
   
   fn main() {
       make();
   }
   ```

1. Create the `build.rs` ( Adapting MacOS ).

   ```rust,no_run
   fn main() {
      #[cfg(target_os = "macos")]
      {
         println!("cargo:rustc-link-arg=-undefined");
         println!("cargo:rustc-link-arg=dynamic_lookup");
      }
   }
   ```

1. Write you owned extension logic in `lib.rs`.

   ```rust
   use phper::{php_get_module, modules::Module};
   
   #[php_get_module]
   pub fn get_module() -> Module {
       let mut module = Module::new(
           env!("CARGO_PKG_NAME"),
           env!("CARGO_PKG_VERSION"),
           env!("CARGO_PKG_AUTHORS"),
       );
   
       // ...
   
       module
   }
   ```

1. Build and install, if your php isn't installed globally, you should specify the path of `php-config`.

   ```bash
   # Optional, specify if php isn't installed globally.
   # export PHP_CONFIG=<Your path of php-config>
   
   # Build libmyapp.so.
   cargo build --release
   
   # Install to php extension path.
   cargo run --release -- install
   # Or if you install php globally, you should use sudo.
   # sudo ./target/release/myapp install
   
   ```

1. Edit your `php.ini`, add the below line.

   ```ini
   extension = myapp
   ```

1. Enjoy.

## Examples

See [examples](https://github.com/phper-framework/phper/tree/master/examples).

## The projects using PHPER

- [apache/skywalking-php](https://github.com/apache/skywalking-php) - The PHP Agent for Apache SkyWalking, which provides the native tracing abilities for PHP project.

## License

[MulanPSL-2.0](https://github.com/phper-framework/phper/blob/master/LICENSE).
