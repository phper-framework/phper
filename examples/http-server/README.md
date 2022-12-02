# http-server

Http server example.

Power by [tokio](https://crates.io/crates/tokio) and [hyper](https://crates.io/crates/hyper).

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
cp target/release/libhttp_server.so `${PHP_CONFIG:=php-config} --extension-dir`
```

## License

[MulanPSL-2.0](https://github.com/phper-framework/phper/blob/master/LICENSE).
