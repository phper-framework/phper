# phper-test

Integration test tool for [phper](https://crates.io/crates/phper).

The `php-config` is needed. You can set environment `PHP_CONFIG` to specify the path.

## Notice

1. Because the `phper-test` depends on the `cdylib` to do integration test, but now `cargo test` don't build `cdylib` in `[lib]`, so you must call `cargo build` before `cargo test`.

   Maybe, when the feature [artifact-dependencies](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies) becomes stable, or the [issue](https://github.com/rust-lang/cargo/issues/8628) be solved, you don't have to call `cargo build` in advance, but I think it will be a long long stage.

1. Or, define an `[[example]]` section, re-export all the symbols of your crate, and set `lto = true`. It's strange, but this is the only method to just run `cargo test` without running `cargo build` in advance. Please refer to [tests/integration/Cargo.toml](https://github.com/phper-framework/phper/blob/master/tests/integration/Cargo.toml).

## License

[MulanPSL-2.0](https://github.com/phper-framework/phper/blob/master/LICENSE).
