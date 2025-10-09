# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.17.1](https://github.com/phper-framework/phper/compare/phper-v0.17.0...phper-v0.17.1) - 2025-10-09

### Fixed

- Update feature attribute for documentation configuration ([#224](https://github.com/phper-framework/phper/pull/224))
- fix invocation of functions with type hint ([#220](https://github.com/phper-framework/phper/pull/220))

### Other

- adding functions for interacting with execution context ([#221](https://github.com/phper-framework/phper/pull/221))
- adding function start and end line number ([#219](https://github.com/phper-framework/phper/pull/219))
- provide access to return_value, function's type, filename, line number ([#217](https://github.com/phper-framework/phper/pull/217))

## [0.17.0](https://github.com/phper-framework/phper/compare/phper-v0.16.1...phper-v0.17.0) - 2025-07-03

### Added

- Add raw pointer casting methods for EBox ([#213](https://github.com/phper-framework/phper/pull/213))
- Add HTML logo URL to documentation for improved branding ([#210](https://github.com/phper-framework/phper/pull/210))

### Fixed

- Fix ZArr drop ([#212](https://github.com/phper-framework/phper/pull/212))
- Update README structure for better alignment and presentation ([#209](https://github.com/phper-framework/phper/pull/209))

### Other

- Migration of ZString, ZArray, ZObject to EBox ([#208](https://github.com/phper-framework/phper/pull/208))

## [0.16.1](https://github.com/phper-framework/phper/compare/phper-v0.16.0...phper-v0.16.1) - 2025-05-01

### Added

- Introduce new_persistent method for ZString ([#204](https://github.com/phper-framework/phper/pull/204))
- Enhance enum functionality with access methods ([#203](https://github.com/phper-framework/phper/pull/203))
- Add preliminary support for enums ([#201](https://github.com/phper-framework/phper/pull/201))

## [0.16.0](https://github.com/phper-framework/phper/compare/phper-v0.15.1...phper-v0.16.0) - 2025-04-04

### Added

- [**breaking**] rename `bind_*` to `bound_*` ([#192](https://github.com/phper-framework/phper/pull/192))

### Other

- allow static interface methods ([#198](https://github.com/phper-framework/phper/pull/198))
- update readme and remove dep once_cell ([#196](https://github.com/phper-framework/phper/pull/196))
- optimizing `extends` and `implements` ([#193](https://github.com/phper-framework/phper/pull/193))
- improve Interface::extends and ClassEntry::extends ([#190](https://github.com/phper-framework/phper/pull/190))
- refactor ClassEntity.implements ([#189](https://github.com/phper-framework/phper/pull/189))
- [breaking] add argument and return value type-hints ([#187](https://github.com/phper-framework/phper/pull/187))
