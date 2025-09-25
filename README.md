# assert-text

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

the testing macro tools.

This checks that strings are equal.
You will see different characters if that is different.

## Features

- assert_text_eq!(txt1, txt2)
- assert_text_starts_with!(txt1, txt2)
- assert_text_ends_with!(txt1, txt2)
- assert_text_match!(txt1, regex_text2)
- minimum support rustc 1.65.0 (897e37553 2022-11-02)


# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/assert-text/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/assert-text.svg
[crate-link]: https://crates.io/crates/assert-text
[docs-image]: https://docs.rs/assert-text/badge.svg
[docs-link]: https://docs.rs/assert-text/
[rustc-image]: https://img.shields.io/badge/rustc-1.65+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/assert-text/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/assert-text/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/assert-text/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/assert-text/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/assert-text/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/assert-text/actions/workflows/test-windows.yml
