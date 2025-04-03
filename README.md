# PHPER (PHP Enjoy Rust)

[![CI](https://github.com/phper-framework/phper/actions/workflows/ci.yml/badge.svg)](https://github.com/phper-framework/phper/actions/workflows/ci.yml)
[![Crates](https://img.shields.io/crates/v/phper)](https://crates.io/crates/phper)
[![Docs](https://img.shields.io/docsrs/phper)](https://docs.rs/phper)
[![License](https://img.shields.io/crates/l/phper)](https://github.com/phper-framework/phper/blob/master/LICENSE)

## Rust ❤️ PHP

The framework that allows us to write PHP extensions using pure and safe Rust whenever possible.

## Documentation & Tutorial

- Documentation: <https://docs.rs/phper>
- Tutorial: <https://docs.rs/phper-doc/>

## Requirement

### Necessary

- **rust** 1.85 or later
- **libclang** 9.0 or later
- **php** 7.0 or later

### Tested Support

| **Category**    | **Item**  | **Status** |
| --------------- | --------- | ---------- |
| **OS**          | Linux     | ✅          |
|                 | macOS     | ✅          |
|                 | Windows   | ❌          |
| **PHP Version** | 7.0 ~ 7.4 | ✅          |
|                 | 8.0 ~ 8.4 | ✅          |
| **PHP Mode**    | NTS       | ✅          |
|                 | ZTS       | ❌          |
| **SAPI**        | CLI       | ✅          |
|                 | FPM       | ✅          |
| **Debug**       | Disable   | ✅          |
|                 | Enable    | ❌          |

## Examples

See [examples](https://github.com/phper-framework/phper/tree/master/examples).

## The projects using PHPER

- [apache/skywalking-php](https://github.com/apache/skywalking-php) - The PHP Agent for Apache SkyWalking, which provides the native tracing abilities for PHP project.

- [phper-framework/jieba-php](https://github.com/phper-framework/jieba-php) - The Jieba Chinese Word Segmentation Implemented in Rust Bound for PHP.

## License

[MulanPSL-2.0](https://github.com/phper-framework/phper/blob/master/LICENSE).
