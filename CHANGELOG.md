# Changelog: assert-text

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.2.10] (2024-06-09)
### Changed
* test and build support 1.65.0 on github workflows

### Fixed
* fixed test: thread_panic_error_out_s

## [0.2.9] (2023-02-12)
### Changed
* `assert_text_match!()`: more message on error
* refactored `Makefile`

### Removed
* `COPYING`

### Fixed
* `LICENSE-APACHE`, `LICENSE-MIT`

## [0.2.8] (2023-01-31)
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`
* `rust-version = "1.56.0"` into `Cargo.toml`

### Fixed
* bug: test result failed on windows
* clippy: `single_component_path_imports`, `redundant_static_lifetimes`
* clippy: `needless_borrow`, `bool_assert_comparison`

## [0.2.7] (2023-01-10)
### Added
* badges into `README.md`

### Changed
* reformat `CHANGELOG.md`
* update crates: regex(1.7)

## [0.2.6] (2022-06-12)
### Changed
* changes to edition 2021

## [0.2.5] (2021-11-14)
### Added
* more documents

## [0.2.4] (2021-07-03)
### Changed
* to github.com

## 0.2.3 (2021-07-03)
### Added
* documents

### Changed
* update depends: exec-target(0.2)
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_test-helper")`
* minimum support rustc 1.43.0

## 0.2.2 (2021-06-23)
### Changed
* update depends: regex(1.5)

## 0.2.1 (2021-04-02)
### Changed
* update depend: exec-target

## 0.2.0 (2021-04-02)
### Changed
* fixed by clippy, remove this semicolon

## 0.1.5 (2020-11-17)
### Added
* README.md, COPYING, LICENSE-APACHE, LICENSE-MIT
* support rustc 1.41

### Changed
* test-helper: downgrade rustc_version

## 0.1.4 (2020-10-09)
### Added
* assert_text_match!()

### Changed
* change edition 2015 to 2018

## 0.1.3 (2018-05-03)
### Added
* support of workspace and cargo make

### Fixed
* bug: fn fotmat_diff_add_rem()

## 0.1.2 (2018-03-23)
### Changed
* update difference

## 0.1.1 (2018-03-22)
### Changed
* a lot of things...

## 0.1.0 (2017-11-21)
* first commit

[Unreleased]: https://github.com/aki-akaguma/assert-text/compare/v0.2.10..HEAD
[0.2.10]: https://github.com/aki-akaguma/assert-text/compare/v0.2.9..v0.2.10
[0.2.9]: https://github.com/aki-akaguma/assert-text/compare/v0.2.8..v0.2.9
[0.2.8]: https://github.com/aki-akaguma/assert-text/compare/v0.2.7..v0.2.8
[0.2.7]: https://github.com/aki-akaguma/assert-text/compare/v0.2.6..v0.2.7
[0.2.6]: https://github.com/aki-akaguma/assert-text/compare/v0.2.5..v0.2.6
[0.2.5]: https://github.com/aki-akaguma/assert-text/compare/v0.2.4..v0.2.5
[0.2.4]: https://github.com/aki-akaguma/assert-text/releases/tag/v0.2.4
