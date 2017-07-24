# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]
### Added
- Following sum with Hasher & Digest implementations:
    * Unix (UNIX `cksum`)
    * FNV 32 in several variants: `FNV1-32`, `FNV1a-32` and `FNV0-32`
- Paper & reference about CRC32, CRC32c, Adler32 and UNIX checksum.
### Fixed
- **CRC32/CRC32C**: Useless NOT operations before/after each block write, replaced by
  a single NOT operation in the `finish` method (and initialisation to `0xFFFFFFFF`).
### Changed
- Implementation of unit tests to use private macros module
- Names of checksum structs to follow Rust naming conventions (acronyms as single
  CamelCased word): `BSD` -> `Bsd`, `CRC32(C)` -> `Crc32(c)`, `UNIX` -> `Unix`
- Code using bitwise operations to use Rust primitive methods where applicable.
- Structure of the library to nest the `crc32` module inside a `crc` module.
## Removed
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
- Tests and benchmarks generated with the [`crypto-tests`](https://crates.io/crates/crypto-tests) crate
- A basic README inspired by the README of the [`hashes`](https://github.com/RustCrypto/hashes) project
- This CHANGELOG file


[Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/0.1.0...HEAD
