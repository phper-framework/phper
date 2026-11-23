# complex

Complex example.

## Environment

```bash
# Optional, specify if php isn't installed globally.
export PHP_CONFIG=<Your path of php-config>
```

## Build

```bash
cargo build --release
```

## Test

```bash
cargo test --release
```

## Install

```bash
cp target/release/libcomplex.so `${PHP_CONFIG:=php-config} --extension-dir`
```

## License

Licensed under your choice of:

- [Mulan Permissive Software License, Version 2](https://github.com/phper-framework/phper/blob/master/LICENSE-MULAN) (`MulanPSL-2.0`)
- [Apache License, Version 2.0](https://github.com/phper-framework/phper/blob/master/LICENSE-APACHE) (`Apache-2.0`)
- [MIT License](https://github.com/phper-framework/phper/blob/master/LICENSE-MIT) (`MIT`)

SPDX expression: `MulanPSL-2.0 OR Apache-2.0 OR MIT`.
