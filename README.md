<div align="center">

<h1>PHPER (PHP Enjoy Rust)</h1>

<img src="https://avatars.githubusercontent.com/u/112468984?s=380&v=4" alt="PHPER Logo">

<p>
<a href="https://github.com/phper-framework/phper/actions/workflows/ci.yml"><img src="https://github.com/phper-framework/phper/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
<a href="https://crates.io/crates/phper"><img src="https://img.shields.io/crates/v/phper" alt="Crates"></a>
<a href="https://docs.rs/phper"><img src="https://img.shields.io/docsrs/phper" alt="Docs"></a>
<a href="https://github.com/phper-framework/phper/blob/master/LICENSE"><img src="https://img.shields.io/crates/l/phper" alt="License"></a>
</p>

</div>

## Rust ❤️ PHP

The framework that allows us to write PHP extensions using pure and safe Rust whenever possible.

## Documentation & Tutorial

- Website: <https://phper-framework.github.io/phper/>
- Tutorial: <https://phper-framework.github.io/phper/docs/>
- API documentation: <https://docs.rs/phper>

## Requirement

### Necessary

- **rust** 1.85 or later
- **libclang** 9.0 or later
- **php** 7.1 or later

### Tested Support

| **Category**    | **Item**  | **Status** |
| --------------- | --------- | ---------- |
| **OS**          | Linux     | ✅         |
|                 | macOS     | ✅         |
|                 | Windows   | ❌         |
| **PHP Version** | 7.1 ~ 7.4 | ✅         |
|                 | 8.0 ~ 8.6 | ✅         |
| **PHP Mode**    | NTS       | ✅         |
|                 | ZTS       | ✅         |
| **SAPI**        | CLI       | ✅         |
|                 | FPM       | ✅         |
| **Debug**       | Disable   | ✅         |
|                 | Enable    | ✅         |

## Examples

See [examples](https://github.com/phper-framework/phper/tree/master/examples).

## The projects using PHPER

- [apache/skywalking-php](https://github.com/apache/skywalking-php) - The PHP Agent for Apache SkyWalking, which provides the native tracing abilities for PHP project.

- [phper-framework/jieba-php](https://github.com/phper-framework/jieba-php) - The Jieba Chinese Word Segmentation Implemented in Rust Bound for PHP.

## License

[MulanPSL-2.0](https://github.com/phper-framework/phper/blob/master/LICENSE).
