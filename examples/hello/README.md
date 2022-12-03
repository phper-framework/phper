# hello

Hello world example.

## Environment

```bash
# Optional, specify if php isn't installed globally.
export PHP_CONFIG=<Your path of php-config>
```

## Build

```bash
cargo build --release
```

## Run

```bash
php -d "extension=target/debug/libhello.so" -r "say_hello('Bob');"
```

## License

[MulanPSL-2.0](https://github.com/phper-framework/phper/blob/master/LICENSE).
