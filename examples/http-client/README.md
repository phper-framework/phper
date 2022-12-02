# http-client

Http client example.

Power by [reqwest::blocking](https://docs.rs/reqwest/0.11.4/reqwest/blocking/index.html) api.

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
cp target/release/libhttp_client.so `${PHP_CONFIG:=php-config} --extension-dir`
```

## License

[MulanPSL-2.0](https://github.com/phper-framework/phper/blob/master/LICENSE).
