# PHPER (PHP Enjoy Rust)

[![CI](https://github.com/phper-framework/phper/actions/workflows/ci.yml/badge.svg)](https://github.com/phper-framework/phper/actions/workflows/ci.yml)
[![Crates](https://img.shields.io/crates/v/phper)](https://crates.io/crates/phper)
[![Docs](https://img.shields.io/docsrs/phper)](https://docs.rs/phper)
[![Lines](https://img.shields.io/tokei/lines/github/phper-framework/phper)](https://github.com/phper-framework/phper)
[![License](https://img.shields.io/crates/l/phper)](https://github.com/phper-framework/phper/blob/master/LICENSE)

## Rust ❤️ PHP

The framework that allows us to write PHP extensions using pure and safe Rust whenever possible.

## Documentation & Tutorial

- Documentation: <https://docs.rs/phper>
- Tutorial: <https://docs.rs/phper-doc/>

## Requirement

### Necessary

- **rust** 1.67 or later
- **libclang** 9.0 or later
- **php** 7.0 or later

### Tested Support

- **OS**
  - [x] linux
  - [x] macos
  - [ ] ~~windows~~
- **PHP**
  - **version**
    - [x] 7.0
    - [x] 7.1
    - [x] 7.2
    - [x] 7.3
    - [x] 7.4
    - [x] 8.0
    - [x] 8.1
    - [x] 8.2
  - **mode**
    - [x] nts
    - [ ] ~~zts~~
  - **sapi**
    - [x] cli
    - [x] fpm
  - **debug**
    - [x] disable
    - [ ] ~~enable~~

## Examples

See [examples](https://github.com/phper-framework/phper/tree/master/examples).

## The projects using PHPER

- [apache/skywalking-php](https://github.com/apache/skywalking-php) - The PHP Agent for Apache SkyWalking, which provides the native tracing abilities for PHP project.

## License

[MulanPSL-2.0](https://github.com/phper-framework/phper/blob/master/LICENSE).
