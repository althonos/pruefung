# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]
### Added
- Following sum with Hasher & Digest implementations:
    * UNIX (UNIX `cksum`)
### Fixed
- **CRC32/CRC32C**: Useless NOT operations before/after each block write, replaced by
  a single NOT operation in the `finish` method (and initialisation to `0xFFFFFFFF`).
### Changed
- Implemented unit tests using private macros module


## 0.1.0 - 2017-07-20
### Added
- Following sums with Hasher & Digest implementations:
    * Adler32
    * BSD checksum (UNIX `sum`)
    * CRC32 (Ethernet variant)
    * CRC32C (Castagnoli polynomial variant)
    * Fletcher16
    * SysV checksum (UNIX `sum -s`)
- Tests and benchmarks generated with the [`crypto-tests`](https://crates.io/crates/crypto-tests) crate
- A basic README inspired by the README of the [`hashes`](https://github.com/RustCrypto/hashes) project
- This CHANGELOG file

[Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/0.1.0...HEAD
