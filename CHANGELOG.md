# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased

### Added
- Implementation of the following sums:
    * Adler32
    * BSD checksum (UNIX `sum`)
    * CRC32 (Ethernet variant)
    * CRC32C (Castagnoli polynomial variant)
    * Fletcher16
    * SysV checksum
- Tests and benchmarks generated with the
  [`crypto-tests`](https://crates.io/crates/crypto-tests) crate
- A basic README inspired by the README of the
  [hashes](https://github.com/RustCrypto/hashes) project
- This CHANGELOG file
