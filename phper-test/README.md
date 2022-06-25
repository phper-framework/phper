# phper-test

Integration test tool for [phper](https://crates.io/crates/phper).

The `php-config` is needed. You can set environment `PHP_CONFIG` to specify the path.

## Notice

Because the `phper-test` depends on the `cdylib` to do integration test, but now `cargo test` don't build `cdylib` in `[lib]`, so you must call `cargo build` before `cargo test`.

Maybe, when the feature [artifact-dependencies](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies) becomes stable, or the [issue](https://github.com/rust-lang/cargo/issues/8628) be solved, you don't have to call `cargo build` in advance, but I think it will be a long long stage.

## License

[MulanPSL-2.0](https://github.com/jmjoy/phper/blob/master/LICENSE).
