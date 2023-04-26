# http-server

Http server example.

Power by [tokio](https://crates.io/crates/tokio) and [axum](https://crates.io/crates/axum).

Because PHP is a single threaded model (NTS), so tokio runtime uses current thread scheduler.

This is just a demo program, and if it want to be as powerful as `swoole`,
it need to work hard on multiprocessing and asynchronous components.

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

## Run

```bash
# Start web server:
php -d "extension=target/release/libhttp_server.so" examples/http-server/tests/php/test.php

# Request:
curl -i http://127.0.0.1:9000/
```

## License

[MulanPSL-2.0](https://github.com/phper-framework/phper/blob/master/LICENSE).
