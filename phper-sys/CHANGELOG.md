# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.0](https://github.com/phper-framework/phper/compare/phper-sys-v0.15.6...phper-sys-v0.16.0) - 2026-11-17

### Added

- [**breaking**] add union types to arguments and returns ([#250](https://github.com/phper-framework/phper/pull/250))
- add typed and readonly class properties ([#249](https://github.com/phper-framework/phper/pull/249))
- [**breaking**] support defaults on class and untyped args ([#248](https://github.com/phper-framework/phper/pull/248))

### Fixed

- pass log messages as an argument, not a format ([#265](https://github.com/phper-framework/phper/pull/265))
- [**breaking**] bump bindgen to 0.73.2 for libclang 22 ([#259](https://github.com/phper-framework/phper/pull/259))
- preserve by-reference on class-typed arguments and return types ([#247](https://github.com/phper-framework/phper/pull/247))

### Other

- Revert materialize_missing feature, refactor CI workflow ([#243](https://github.com/phper-framework/phper/pull/243))

## [0.15.6](https://github.com/phper-framework/phper/compare/phper-sys-v0.15.5...phper-sys-v0.15.6) - 2026-06-25

### Added

- add materialize_missing for optional parameters ([#239](https://github.com/phper-framework/phper/pull/239))

## [0.15.5](https://github.com/phper-framework/phper/compare/phper-sys-v0.15.4...phper-sys-v0.15.5) - 2026-06-23

### Other

- Refactor type hints, improve CI workflows, and enhance safety comments ([#237](https://github.com/phper-framework/phper/pull/237))

## [0.15.4](https://github.com/phper-framework/phper/compare/phper-sys-v0.15.3...phper-sys-v0.15.4) - 2025-10-09

### Other

- adding functions for interacting with execution context ([#221](https://github.com/phper-framework/phper/pull/221))

## [0.15.3](https://github.com/phper-framework/phper/compare/phper-sys-v0.15.2...phper-sys-v0.15.3) - 2025-07-03

### Added

- Add HTML logo URL to documentation for improved branding ([#210](https://github.com/phper-framework/phper/pull/210))

## [0.15.2](https://github.com/phper-framework/phper/compare/phper-sys-v0.15.1...phper-sys-v0.15.2) - 2025-05-01

### Added

- Add preliminary support for enums ([#201](https://github.com/phper-framework/phper/pull/201))

### Fixed

- fix ClassEntry typehints ([#206](https://github.com/phper-framework/phper/pull/206))

## [0.15.1](https://github.com/phper-framework/phper/compare/phper-sys-v0.15.0...phper-sys-v0.15.1) - 2025-04-04

### Other

- Remove workspace.package.version ([#199](https://github.com/phper-framework/phper/pull/199))
- [breaking] add argument and return value type-hints ([#187](https://github.com/phper-framework/phper/pull/187))
