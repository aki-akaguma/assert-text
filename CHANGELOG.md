# Changelog: assert-text
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2026-05-27
### Changed
- Update test-helper crate: exec-target(0.3)

## [0.3.0] - 2026-05-26
### Added
- Support custom panic messages in all assertion macros (`assert_text_eq!()`, `assert_text_starts_with!()`, `assert_text_ends_with!()`, `assert_text_contains!()`, `assert_text_match!()`)
- Support NO_COLOR environment variable to disable ANSI colors in diff output

### Fixed
- Prevent UTF-8 slicing panics in `assert_text_starts_with!()` and `assert_text_ends_with!()` by determining slice indices using character boundaries instead of raw byte lengths
- Remove redundant memory allocations and improve initial `String` capacity calculation in diff formatting functions (`format_diff_line_same` and `format_diff_line_mark`)
- Reduce heap allocations in `format_diff_add_rem` by using borrowed string slices instead of creating many small `String` objects

### Changed
- Ensure macro argument hygiene and single evaluation in all assertion macros (`assert_text_eq!()`, `assert_text_starts_with!()`, `assert_text_ends_with!()`, `assert_text_contains!()`, `assert_text_match!()`) by binding arguments to references using `match`

## [0.2.11] - 2025-09-28
### Added
- Documentation comments
- `assert_text_contains!()` macro

### Changed
- Set minimum supported Rust version to 1.65.0
- Refactor tests

## [0.2.10] - 2024-06-09
### Changed
- Support Rust 1.65.0 on GitHub Actions workflows

### Fixed
- Test `thread_panic_error_out_s`

## [0.2.9] - 2023-02-12
### Changed
- Improve error message in `assert_text_match!()`
- Refactor `Makefile`

### Removed
- `COPYING` file

### Fixed
- `LICENSE-APACHE` and `LICENSE-MIT` files

## [0.2.8] - 2023-01-31
### Added
- GitHub Actions workflows for Ubuntu, macOS, and Windows
- Test status badges in `README.tpl`
- Minimum supported Rust version 1.56.0 in `Cargo.toml`

### Fixed
- Test failures on Windows
- Clippy warnings for `single_component_path_imports`, `redundant_static_lifetimes`, `needless_borrow`, and `bool_assert_comparison`

## [0.2.7] - 2023-01-10
### Added
- Badges in `README.md`

### Changed
- Reformat `CHANGELOG.md`
- Update regex crate to 1.7

## [0.2.6] - 2022-06-12
### Changed
- Update to Rust 2021 edition

## [0.2.5] - 2021-11-14
### Added
- Additional documentation

## [0.2.4] - 2021-07-03
### Changed
- Move project to GitHub

## 0.2.3 - 2021-07-03
### Added
- Documentation

### Changed
- Update `exec-target` dependency to 0.2
- Rewrite `TARGET_EXE_PATH` using `env!("CARGO_BIN_EXE_test-helper")`
- Set minimum supported Rust version to 1.43.0

## 0.2.2 - 2021-06-23
### Changed
- Update `regex` dependency to 1.5

## 0.2.1 - 2021-04-02
### Changed
- Update `exec-target` dependency

## 0.2.0 - 2021-04-02
### Changed
- Clippy fixes (removed redundant semicolon)

## 0.1.5 - 2020-11-17
### Added
- `README.md`, `COPYING`, `LICENSE-APACHE`, and `LICENSE-MIT` files
- Support for Rust 1.41

### Changed
- Downgrade `rustc_version` in `test-helper`

## 0.1.4 - 2020-10-09
### Added
- `assert_text_match!()` macro

### Changed
- Update project edition from 2015 to 2018

## 0.1.3 - 2018-05-03
### Added
- Support for workspaces and `cargo make`

### Fixed
- Function `format_diff_add_rem()`

## 0.1.2 - 2018-03-23
### Changed
- Update `difference` dependency

## 0.1.1 - 2018-03-22
### Changed
- Various improvements and updates

## 0.1.0 - 2017-11-21
### Added
- Initial release

[Unreleased]: https://github.com/aki-akaguma/assert-text/compare/v0.3.1..HEAD
[0.3.1]: https://github.com/aki-akaguma/assert-text/compare/v0.3.0..v0.3.1
[0.3.0]: https://github.com/aki-akaguma/assert-text/compare/v0.2.11..v0.3.0
[0.2.11]: https://github.com/aki-akaguma/assert-text/compare/v0.2.10..v0.2.11
[0.2.10]: https://github.com/aki-akaguma/assert-text/compare/v0.2.9..v0.2.10
[0.2.9]: https://github.com/aki-akaguma/assert-text/compare/v0.2.8..v0.2.9
[0.2.8]: https://github.com/aki-akaguma/assert-text/compare/v0.2.7..v0.2.8
[0.2.7]: https://github.com/aki-akaguma/assert-text/compare/v0.2.6..v0.2.7
[0.2.6]: https://github.com/aki-akaguma/assert-text/compare/v0.2.5..v0.2.6
[0.2.5]: https://github.com/aki-akaguma/assert-text/compare/v0.2.4..v0.2.5
[0.2.4]: https://github.com/aki-akaguma/assert-text/releases/tag/v0.2.4
