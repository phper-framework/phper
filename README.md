# PHPer

A library that allows us to write PHP extensions in pure Rust, using safe Rust where possible, and also a PHP Binding library.

(一个让我们可以用纯Rust写PHP扩展的库，并且尽可能使用safe Rust，同时也是一个PHP的Binding库。)

***Now the peojct is still under development.***

## Usage

First you have to install `cargo-generate`:

```bash
cargo install cargo-generate
```

Then create a PHP extension project from template:

```bash
cargo generate --git https://github.com/jmjoy/phper-ext-skel.git
```

