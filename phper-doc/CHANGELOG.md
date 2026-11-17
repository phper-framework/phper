# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.0](https://github.com/phper-framework/phper/compare/phper-doc-v0.15.8...phper-doc-v0.16.0) - 2026-11-17

### Added

- add typed static properties ([#251](https://github.com/phper-framework/phper/pull/251))
- [**breaking**] add union types to arguments and returns ([#250](https://github.com/phper-framework/phper/pull/250))
- add typed and readonly class properties ([#249](https://github.com/phper-framework/phper/pull/249))
- [**breaking**] support defaults on class and untyped args ([#248](https://github.com/phper-framework/phper/pull/248))
- add ClassEntity::set_final to declare a class final ([#246](https://github.com/phper-framework/phper/pull/246))

### Fixed

- declare iterable arguments untyped on PHP 7.0 ([#263](https://github.com/phper-framework/phper/pull/263))
- [**breaking**] bump bindgen to 0.73.2 for libclang 22 ([#259](https://github.com/phper-framework/phper/pull/259))
- prevent abort when a required argument follows an optional one ([#244](https://github.com/phper-framework/phper/pull/244))

### Other

- *(deps)* update all dependencies ([#257](https://github.com/phper-framework/phper/pull/257))

## [0.15.8](https://github.com/phper-framework/phper/compare/phper-doc-v0.15.7...phper-doc-v0.15.8) - 2026-06-26

### Other

- updated the following local packages: phper

## [0.15.7](https://github.com/phper-framework/phper/compare/phper-doc-v0.15.6...phper-doc-v0.15.7) - 2026-06-25

### Other

- updated the following local packages: phper, phper-build

## [0.15.6](https://github.com/phper-framework/phper/compare/phper-doc-v0.15.5...phper-doc-v0.15.6) - 2026-06-23

### Added

- Update configuration for PHP 8.1+ enum support and adjust linting ([#227](https://github.com/phper-framework/phper/pull/227))

### Other

- *(zval)* add `FromZVal` / `FromZValMut` guidance in `_04_zval` ([#235](https://github.com/phper-framework/phper/pull/235))

## [0.15.5](https://github.com/phper-framework/phper/compare/phper-doc-v0.15.4...phper-doc-v0.15.5) - 2025-12-08

### Other

- updated the following local packages: phper

## [0.15.4](https://github.com/phper-framework/phper/compare/phper-doc-v0.15.3...phper-doc-v0.15.4) - 2025-10-09

### Fixed

- Update feature attribute for documentation configuration ([#224](https://github.com/phper-framework/phper/pull/224))

### Other

- adding function start and end line number ([#219](https://github.com/phper-framework/phper/pull/219))

## [0.15.3](https://github.com/phper-framework/phper/compare/phper-doc-v0.15.2...phper-doc-v0.15.3) - 2025-07-03

### Added

- Add HTML logo URL to documentation for improved branding ([#210](https://github.com/phper-framework/phper/pull/210))

### Other

- Migration of ZString, ZArray, ZObject to EBox ([#208](https://github.com/phper-framework/phper/pull/208))

## [0.15.2](https://github.com/phper-framework/phper/compare/phper-doc-v0.15.1...phper-doc-v0.15.2) - 2025-05-01

### Added

- Enhance enum functionality with access methods ([#203](https://github.com/phper-framework/phper/pull/203))
- Add preliminary support for enums ([#201](https://github.com/phper-framework/phper/pull/201))

### Other

- Adjust the documentation of the enum ([#202](https://github.com/phper-framework/phper/pull/202))

## [0.15.1](https://github.com/phper-framework/phper/compare/phper-doc-v0.15.0...phper-doc-v0.15.1) - 2025-04-04

### Other

- Remove workspace.package.version ([#199](https://github.com/phper-framework/phper/pull/199))
- allow static interface methods ([#198](https://github.com/phper-framework/phper/pull/198))
- test and document co-dependent classes ([#194](https://github.com/phper-framework/phper/pull/194))
- improve Interface::extends and ClassEntry::extends ([#190](https://github.com/phper-framework/phper/pull/190))
- refactor ClassEntity.implements ([#189](https://github.com/phper-framework/phper/pull/189))
- [breaking] add argument and return value type-hints ([#187](https://github.com/phper-framework/phper/pull/187))
