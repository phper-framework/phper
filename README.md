# PHPer

[![CI](https://github.com/jmjoy/phper/actions/workflows/ci.yml/badge.svg)](https://github.com/jmjoy/phper/actions/workflows/ci.yml)
[![Crates](https://img.shields.io/crates/v/phper)](https://crates.io/crates/phper)
[![Docs](https://img.shields.io/docsrs/phper)](https://docs.rs/phper)
[![Lines](https://img.shields.io/tokei/lines/github/jmjoy/phper)](https://github.com/jmjoy/phper)
[![License](https://img.shields.io/crates/l/phper)](https://github.com/jmjoy/phper/blob/master/LICENSE)

## Rust ❤️ PHP

A library that allows us to write PHP extensions using pure Rust and using safe Rust whenever possible.

## Requirement

### Necessary

- **rust** 1.56 or later
- **libclang** 9.0 or later
- **php** 7.0 or later

### Tested Support

- **OS**
    - [x] linux
    - [ ] macos
    - [ ] windows
- **PHP**
  - **version**
    - [x] 7.0
    - [x] 7.1
    - [x] 7.2
    - [x] 7.3
    - [x] 7.4
    - [x] 8.0
    - [ ] 8.1
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

2. Create you cargo project, suppose your application is called myapp.

```bash
cargo new myapp
```

3. Add the dependencies and metadata to you Cargo project.

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
phper = "0.3"
```

4. Add these code to `main.rs`.

```rust,no_run
use phper::cmd::make;

fn main() {
    make();
}
```

5. Write you owned extension logic in `lib.rs`.

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

6. Build and install, if your php isn't installed globally, you should specify the path of `php-config`.

```bash
# Optional, specify if php isn't installed globally.
export PHP_CONFIG=<Your path of php-config>

# Build libmyapp.so.
cargo build --release

# Install to php extension path.
cargo run --release -- install
# Or if you install php globally, you should use sudo.
# sudo ./target/release/myapp install

```

7. Edit your `php.ini`, add the below line.

```ini
extension = myapp
```

8. Enjoy.

## Examples

See [examples](https://github.com/jmjoy/phper/tree/master/examples).

## License

[Unlicense](https://github.com/jmjoy/phper/blob/master/LICENSE).
