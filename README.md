# PHPer

[![crates](https://img.shields.io/crates/v/phper?style=flat-square)](https://crates.io/crates/phper)
[![](https://img.shields.io/docsrs/phper?style=flat-square)](https://docs.rs/phper)

A library that allows us to write PHP extensions using pure Rust and using safe Rust whenever possible.

## Requirement

### Necessary

**libclang** version >= 9

**php** version >= 7

### Tested Support

**os**

- linux

**php**

*version*

- 7.0
- 7.1
- 7.2
- 7.3
- 7.4
- 8.0

*mode*

- nts

*sapi*

- cli

## Usage

1. Make sure `libclang` and `php` is installed.

```bash
# If you are using debian like linux system:
sudo apt install libclang-10-dev php-cli
```

2. Create you cargo project, suppose your application is called myapp.

```bash
cargo new myapp
```

3. Add the dependencies and metadata to you Cargo project.

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
phper = "0.2"
```

4. Add these code to `main.rs`.

```rust
use phper::cmd::make;

fn main() {
    make();
}
```

5. Write you owned extension logic in `lib.rs`.

```rust
use phper::{php_get_module, modules::Module};

#[php_get_module]
pub fn get_module(module: &mut Module) {
    // set module metadata
    module.set_name(env!("CARGO_PKG_NAME"));
    module.set_version(env!("CARGO_PKG_VERSION"));
    module.set_author(env!("CARGO_PKG_AUTHORS"));

    // ...
}
```

6. Build and install, if your php isn't installed globally, you should specify the path of `php-config`.

```bash
# Specify if php isn't installed globally.
export PHP_CONFIG = <Your path of php-config>

# Build libmyapp.so.
cargo build --release

# Install to php extension path, if you install php globally, you should use sudo.
cargo run --release -- install
```

7. Edit your `php.ini`, add the below line.

```ini
extension = myapp
```

8. Enjoy.

## examples

See [examples](https://github.com/jmjoy/phper/tree/master/examples).

## License

[Unlicense](https://github.com/jmjoy/phper/blob/master/LICENSE).
