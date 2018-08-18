# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]
### Added
- Following sum with `Hasher` and `Digest` implementation ([#3](https://github.com/althonos/pruefung/pull/3)):
    * Crc8 (CCIT variant)
    * Crc16 (XMODEM variant)
    * Crc64 (*???* variant)
- Derived `Debug` implementation for all `Hasher`.
### Changed
- Bumped required `digest` version to `v0.7`.
- Bumped required `generic-array` version to `v0.9`.
### Removed
- [`crypto-test`](https://crates.io/crates/crypto-tests) test dependency
  (replaced by [`digest::dev`](https://docs.rs/crate/digest/0.7.5/source/src/dev.rs)).

## [0.2.1] - 2017-11-23
### Added
- `Digest` and `Hasher` traits are re-exported at root of module.
### Fixed
- `README.md` is now the readme file of the crate.
- Make `BSD` checksum available only since `0.2.1`.

## [0.2.0] - 2017-07-24
### Added
- Following sum with `Hasher` & `Digest` implementations:
    * Unix (UNIX `cksum`)
    * FNV 32 in several variants: `FNV1-32`, `FNV1a-32` and `FNV0-32`
    * FNV 64 in several variants: `FNV1-64`, `FNV1a-64` and `FNV0-64`
- Paper & reference about CRC32, CRC32c, Adler32 and UNIX checksum.
### Fixed
- **CRC32/CRC32C**: Useless NOT operations before/after each block write, replaced by
  a single NOT operation in the `finish` method (and initialisation to `0xFFFFFFFF`).
### Changed
- Implementation of unit tests to use private macros module.
- Names of checksum structs to follow Rust naming conventions (acronyms as single
  CamelCased word): `BSD` -> `Bsd`, `CRC32(C)` -> `Crc32(c)`, `UNIX` -> `Unix`.
- Code using bitwise operations to use Rust primitive methods where applicable.
- Structure of the library to nest the `crc32` module inside a `crc` module.
### Removed
- [`byte-tools`](https://crates.io/crates/byte-tools) and [`block-buffer`](https://crates.io/crates/block-buffers)
  dependencies, *crate is now free of any requirement !*


## 0.1.0 - 2017-07-20
### Added
- Following sums with Hasher & Digest implementations:
    * Adler32
    * Bsd checksum (UNIX `sum`)
    * Crc32 (Ethernet variant)
    * Crc32c (Castagnoli polynomial variant)
    * Fletcher16
    * SysV checksum (UNIX `sum -s`)
- Tests and benchmarks generated with the [`crypto-tests`](https://crates.io/crates/crypto-tests) crate.
- A basic README inspired by the README of the [`hashes`](https://github.com/RustCrypto/hashes) project.
- This CHANGELOG file.


[Unreleased]: https://github.com/althonos/pruefung/compare/0.2.0...HEAD
[0.2.0]: https://github.com/althonos/pruefung/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/althonos/pruefung/compare/8b9ae86...0.1.0
